# Intro

bla bla bla

```
	
	MiddleWare	[Authentication/Autorization/Analytics]
		|
		|
		▼
	
	WebAPI [Rust/Axum]		--->		DB(NER+Articles) [PostGreSQL]
	
		|
		|
		▼
		
	WebClient [Dioxus] 
```


## Preparing PostgreSql Server.

### Setup The DB Server

1. Install The PostgreSql server

for Database `Postgresql`

```bash
# Install PostgreSql Localy
sudo pacman -S postgresql

# Set PostgreSql Service to auto and starting it.
sudo systemctl enable --now postgresql
```

2. Specify HardDrive Locationo for the DB and Tables.

We do need to specify the location where PostgreSql will save the db and tables on the disk.
- Step 1: Prepare the target folder

``` bash
mkdir -p /mnt/ssd2/pg_custom_space
chown postgres:postgres /mnt/ssd2/pg_custom_space
```
- Create the tablespace

``` sql
CREATE TABLESPACE fast_storage LOCATION '/mnt/ssd2/pg_custom_space';
```

- Assign a database or table:

+ To create an entire database in that location

``` sql
CREATE DATABASE my_db TABLESPACE fast_storage;
```
	
+ To create a table in that location
	
``` sql
CREATE TABLE logs (id SERIAL, log_text TEXT) TABLESPACE fast_storage;
```

## Extended DB Schema (NER + Translation + Summary)

### Final Vison of The DB Schema

``` sql
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

