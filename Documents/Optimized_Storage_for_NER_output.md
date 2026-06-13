# Optimized Storage for NER output

## Extended Schema (NER + Translation + Summary)

```sql
-- Core article store
CREATE TABLE articles (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    source          TEXT,
    url             TEXT UNIQUE,
    published_at    TIMESTAMPTZ,
    original_lang   TEXT,          -- e.g. 'fr', 'ar', 'en'
    raw_text        TEXT,
    created_at      TIMESTAMPTZ DEFAULT now()
);

-- Translations (one row per language)
CREATE TABLE article_translations (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    article_id      UUID REFERENCES articles(id) ON DELETE CASCADE,
    lang            TEXT NOT NULL,     -- target language, e.g. 'en'
    translated_text TEXT NOT NULL,
    translated_at   TIMESTAMPTZ DEFAULT now(),
    UNIQUE(article_id, lang)
);

-- Summaries (can have one per language)
CREATE TABLE article_summaries (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    article_id      UUID REFERENCES articles(id) ON DELETE CASCADE,
    lang            TEXT NOT NULL,
    summary_text    TEXT NOT NULL,
    summarized_at   TIMESTAMPTZ DEFAULT now(),
    UNIQUE(article_id, lang)
);

-- NER entities
CREATE TABLE entities (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    article_id      UUID REFERENCES articles(id) ON DELETE CASCADE,
    text            TEXT NOT NULL,
    label           TEXT NOT NULL,   -- PERSON, ORG, GPE, DATE ...
    start_char      INT,
    end_char        INT,
    confidence      FLOAT
);

-- Indexes for fast retrieval
CREATE INDEX idx_entities_label     ON entities(label);
CREATE INDEX idx_entities_text      ON entities(text);
CREATE INDEX idx_entities_article   ON entities(article_id);
CREATE INDEX idx_articles_lang      ON articles(original_lang);
CREATE INDEX idx_articles_published ON articles(published_at DESC);
```

## Suggested Processing Pipeline

```textile
Raw Article
    │
    ▼
[1] Store in articles table
    │
    ▼
[2] Detect language  (langdetect / spaCy)
    │
    ▼
[3] Translate → article_translations
    │
    ▼
[4] Summarize → article_summaries
    │
    ▼
[5] Run NER    → entities table
    │
    ▼
[6] Serve via FastAPI
```

### Quick ArchLinux Setup

```bash
# Database
sudo pacman -S postgresql
sudo systemctl enable --now postgresql

# Python environment
sudo pacman -S python python-pip
pip install fastapi uvicorn sqlalchemy spacy psycopg2-binary

# spaCy multilingual model
python -m spacy download en_core_web_sm   # add more languages as needed

# LibreTranslate (translation)
pip install libretranslate
libretranslate --host 0.0.0.0 --port 5000
```

### Full Local Stack on ArchLinux (CPU only)

```textile
┌─────────────────────────────────────────────┐
│              Your ArchLinux Machine         │
│                                             │
│  ┌─────────────┐     ┌─────────────────┐    │
│  │  PostgreSQL │     │  Python Workers │    │
│  │  (database) │◄────│  spaCy / HF NER │    │
│  └─────────────┘     │  LibreTranslate │    │
│         ▲            │  Sumy / HF Summ │    │
│         │            └─────────────────┘    │
│  ┌──────┴──────┐                            │
│  │  FastAPI    │  ◄── REST / JSON API       │
│  │  (backend)  │                            │
│  └──────┬──────┘                            │
│         │                                   │
│  ┌──────▼──────┐                            │
│  │  React /    │  ◄── Web Client            │
│  │  Next.js    │                            │
│  └─────────────┘                            │
└─────────────────────────────────────────────┘
         ▲
    Browser / API consumers
```
