package notes

import (
	"errors"
	"sort"
	"strings"
	"sync"
	"time"
)

var ErrNotFound = errors.New("note not found")

type Note struct {
	ID        string    `json:"id"`
	Title     string    `json:"title"`
	Body      string    `json:"body"`
	CreatedAt time.Time `json:"created_at"`
	UpdatedAt time.Time `json:"updated_at"`
}

type Store struct {
	mu     sync.RWMutex
	nextID int64
	notes  map[string]Note
}

func NewStore() *Store {
	return &Store{
		nextID: 1,
		notes:  make(map[string]Note),
	}
}

func (s *Store) Create(title, body string) Note {
	now := time.Now().UTC()

	s.mu.Lock()
	defer s.mu.Unlock()

	id := formatID(s.nextID)
	s.nextID++

	note := Note{
		ID:        id,
		Title:     strings.TrimSpace(title),
		Body:      strings.TrimSpace(body),
		CreatedAt: now,
		UpdatedAt: now,
	}
	s.notes[id] = note
	return note
}

func (s *Store) List() []Note {
	s.mu.RLock()
	defer s.mu.RUnlock()

	items := make([]Note, 0, len(s.notes))
	for _, note := range s.notes {
		items = append(items, note)
	}

	sort.Slice(items, func(i, j int) bool {
		return items[i].CreatedAt.Before(items[j].CreatedAt)
	})
	return items
}

func (s *Store) Get(id string) (Note, bool) {
	s.mu.RLock()
	defer s.mu.RUnlock()

	note, ok := s.notes[id]
	return note, ok
}

func (s *Store) Delete(id string) error {
	s.mu.Lock()
	defer s.mu.Unlock()

	if _, ok := s.notes[id]; !ok {
		return ErrNotFound
	}
	delete(s.notes, id)
	return nil
}

func formatID(id int64) string {
	return strings.ToLower(time.Unix(id, 0).UTC().Format("20060102150405"))
}
