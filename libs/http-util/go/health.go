package httputil

import (
	"encoding/json"
	"log"
	"net/http"
	"sync/atomic"
)

type Health struct {
	ready *atomic.Bool
}

func NewHealth(ready bool) *Health {
	health := &Health{ready: &atomic.Bool{}}
	health.ready.Store(ready)
	return health
}

func (h *Health) SetReady(ready bool) {
	h.ready.Store(ready)
}

func (h *Health) LivenessHandler(w http.ResponseWriter, _ *http.Request) {
	writeStatus(w, http.StatusOK, "ok")
}

func (h *Health) ReadinessHandler(w http.ResponseWriter, _ *http.Request) {
	if !h.ready.Load() {
		writeStatus(w, http.StatusServiceUnavailable, "not ready")
		return
	}
	writeStatus(w, http.StatusOK, "ok")
}

func writeStatus(w http.ResponseWriter, status int, value string) {
	w.Header().Set("Content-Type", "application/json")
	w.WriteHeader(status)
	if err := json.NewEncoder(w).Encode(map[string]string{"status": value}); err != nil {
		log.Printf("write health response: %v", err)
	}
}
