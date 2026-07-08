package main

import (
	"encoding/json"
	"errors"
	"log"
	"net/http"
	"os"
	"strconv"
	"strings"

	httputil "github.com/scintillavoy/monorepo-example/libs/http-util/go"
	"github.com/scintillavoy/monorepo-example/projects/notes/internal/notes"
	"gopkg.in/yaml.v3"
)

type apiConfig struct {
	ServiceName string `yaml:"service_name"`
	Port        int    `yaml:"port"`
}

type createNoteRequest struct {
	Title string `json:"title"`
	Body  string `json:"body"`
}

type errorResponse struct {
	Error string `json:"error"`
}

func main() {
	config, err := loadConfig()
	if err != nil {
		log.Fatal(err)
	}

	store := notes.NewStore()
	health := httputil.NewHealth(true)
	mux := http.NewServeMux()

	mux.HandleFunc("GET /alive", health.LivenessHandler)
	mux.HandleFunc("GET /ready", health.ReadinessHandler)
	mux.HandleFunc("GET /notes", func(w http.ResponseWriter, r *http.Request) {
		writeJSON(w, http.StatusOK, store.List())
	})
	mux.HandleFunc("POST /notes", func(w http.ResponseWriter, r *http.Request) {
		var req createNoteRequest
		if err := json.NewDecoder(r.Body).Decode(&req); err != nil {
			writeError(w, http.StatusBadRequest, "invalid JSON body")
			return
		}
		if strings.TrimSpace(req.Title) == "" {
			writeError(w, http.StatusBadRequest, "title is required")
			return
		}
		writeJSON(w, http.StatusCreated, store.Create(req.Title, req.Body))
	})
	mux.HandleFunc("GET /notes/{id}", func(w http.ResponseWriter, r *http.Request) {
		note, ok := store.Get(r.PathValue("id"))
		if !ok {
			writeError(w, http.StatusNotFound, "note not found")
			return
		}
		writeJSON(w, http.StatusOK, note)
	})
	mux.HandleFunc("DELETE /notes/{id}", func(w http.ResponseWriter, r *http.Request) {
		if err := store.Delete(r.PathValue("id")); err != nil {
			if errors.Is(err, notes.ErrNotFound) {
				writeError(w, http.StatusNotFound, "note not found")
				return
			}
			writeError(w, http.StatusInternalServerError, "delete note failed")
			return
		}
		w.WriteHeader(http.StatusNoContent)
	})

	addr := ":" + config.port()
	log.Printf("%s listening on %s", config.ServiceName, addr)
	if err := http.ListenAndServe(addr, mux); err != nil {
		log.Fatal(err)
	}
}

func loadConfig() (apiConfig, error) {
	configPath := strings.TrimSpace(os.Getenv("API_CONFIG_PATH"))
	if configPath == "" {
		return apiConfig{}, errors.New("API_CONFIG_PATH is required")
	}

	data, err := os.ReadFile(configPath)
	if err != nil {
		return apiConfig{}, err
	}

	config := apiConfig{
		ServiceName: "notes-api",
		Port:        3001,
	}
	if err := yaml.Unmarshal(data, &config); err != nil {
		return apiConfig{}, err
	}
	return config, nil
}

func (c apiConfig) port() string {
	if c.Port == 0 {
		return "3001"
	}
	return strconv.Itoa(c.Port)
}

func writeJSON(w http.ResponseWriter, status int, value any) {
	w.Header().Set("Content-Type", "application/json")
	w.WriteHeader(status)
	if err := json.NewEncoder(w).Encode(value); err != nil {
		log.Printf("write response: %v", err)
	}
}

func writeError(w http.ResponseWriter, status int, message string) {
	writeJSON(w, status, errorResponse{Error: message})
}
