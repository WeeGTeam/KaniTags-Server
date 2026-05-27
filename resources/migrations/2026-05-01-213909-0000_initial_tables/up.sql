CREATE TYPE auto_tag_status AS ENUM ('PENDING', 'COMPLETE');
CREATE TYPE image_format AS ENUM ('PNG', 'JPG');
CREATE TYPE reverse_lookup_site AS ENUM ('IQDB');
CREATE TYPE source_site_name AS ENUM ('GELBOORU');
CREATE TYPE source_status AS ENUM ('EXISTING', 'MISSING');
CREATE TYPE tag_type AS ENUM ('RATING', 'ARTIST', 'SOURCE', 'CHARACTER', 'GENERAL');

CREATE TABLE user_account
(
    id           BIGSERIAL PRIMARY KEY                              NOT NULL,
    created_at   TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at   TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
    user_name    VARCHAR(40)                                        NOT NULL UNIQUE, -- e.g. authelia user name
    display_name VARCHAR(40)                                        NOT NULL
);

CREATE TABLE image
(
    id              BIGSERIAL PRIMARY KEY                              NOT NULL,
    created_at      TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at      TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
    id_hash         bytea                                              NOT NULL UNIQUE, -- e.g. '3b6368639f3e17fa'
    perceptual_hash bit(144)                                           NOT NULL,        -- e.g. '3803887ff7833837f03e43e43e21303b61fe'
    file_name       VARCHAR(60)                                        NOT NULL,
    image_format    image_format                                       NOT NULL,
    res_width       INT                                                NOT NULL,
    res_height      INT                                                NOT NULL
);
CREATE INDEX idx__image__perceptual_hash ON image USING hnsw (perceptual_hash bit_hamming_ops);

CREATE TABLE image_source
(
    id                  BIGSERIAL PRIMARY KEY                              NOT NULL,
    created_at          TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at          TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
    image_id            BIGINT                                             NOT NULL REFERENCES image (id) ON DELETE CASCADE,
    reverse_lookup_site reverse_lookup_site                                NOT NULL,
    source_site         source_site_name                                   NOT NULL,
    source_status       source_status                                      NOT NULL,
    source_url          VARCHAR(256)                                       NULL,
    certainty           FLOAT                                              NOT NULL,
    UNIQUE (image_id, reverse_lookup_site, source_site)
);

CREATE TABLE user_image
(
    id         BIGSERIAL PRIMARY KEY                              NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
    user_id    BIGINT                                             NOT NULL REFERENCES user_account (id) ON DELETE CASCADE,
    image_id   BIGINT                                             NOT NULL REFERENCES image (id) ON DELETE CASCADE,
    UNIQUE (user_id, image_id)
);

CREATE TABLE tag
(
    id         BIGSERIAL PRIMARY KEY                              NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
    tag_type   tag_type                                           NOT NULL,
    tag_name   VARCHAR(40)                                        NOT NULL,
    UNIQUE (tag_type, tag_name)
);

CREATE TABLE image_tag
(
    id          BIGSERIAL PRIMARY KEY                              NOT NULL,
    created_at  TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at  TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
    image_id    BIGINT                                             NOT NULL REFERENCES image (id) ON DELETE CASCADE,
    tag_id      BIGINT                                             NOT NULL REFERENCES tag (id),
    user_id     BIGINT                                             NULL REFERENCES user_account (id) ON DELETE SET NULL,
    source_site source_site_name                                   NULL,
    UNIQUE (image_id, tag_id)
);

CREATE TABLE collection
(
    id         BIGSERIAL PRIMARY KEY                              NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
    user_id    BIGINT                                             NOT NULL REFERENCES user_account (id) ON DELETE CASCADE,
    name       VARCHAR(40)                                        NOT NULL,
    UNIQUE (user_id, name)
);

CREATE TABLE collection_image
(
    id            BIGSERIAL PRIMARY KEY                              NOT NULL,
    created_at    TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at    TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
    image_id      BIGINT                                             NOT NULL REFERENCES image (id) ON DELETE CASCADE,
    collection_id BIGINT                                             NOT NULL REFERENCES collection (id) ON DELETE CASCADE,
    UNIQUE (image_id, collection_id)
);

CREATE TABLE import_session
(
    id         BIGSERIAL PRIMARY KEY                              NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
    user_id    BIGINT                                             NOT NULL REFERENCES user_account (id) ON DELETE CASCADE,
    closed_at  TIMESTAMP WITH TIME ZONE                           NULL
);

CREATE TABLE import_session_image
(
    id         BIGSERIAL PRIMARY KEY                              NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
    import_id  BIGINT                                             NOT NULL REFERENCES import_session (id) ON DELETE CASCADE,
    image_id   BIGINT                                             NOT NULL REFERENCES image (id) ON DELETE CASCADE,
    UNIQUE (import_id, image_id)
);

CREATE TABLE auto_tag_session
(
    id          BIGSERIAL PRIMARY KEY                              NOT NULL,
    created_at  TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at  TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
    user_id     BIGINT                                             NOT NULL REFERENCES user_account (id) ON DELETE CASCADE,
    lookup_site reverse_lookup_site                                NOT NULL,
    closed_at   TIMESTAMP WITH TIME ZONE                           NULL
);

CREATE TABLE auto_tag_session_image
(
    id         BIGSERIAL PRIMARY KEY                              NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
    session_id BIGINT                                             NOT NULL REFERENCES auto_tag_session (id) ON DELETE CASCADE,
    image_id   BIGINT                                             NOT NULL REFERENCES image (id) ON DELETE CASCADE,
    status     auto_tag_status                                    NOT NULL,
    UNIQUE (session_id, image_id)
);

CREATE TABLE auto_tag_session_image_option
(
    id               BIGSERIAL PRIMARY KEY                              NOT NULL,
    created_at       TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at       TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
    session_image_id BIGINT                                             NOT NULL REFERENCES auto_tag_session_image (id) ON DELETE CASCADE,
    source_site      source_site_name                                   NOT NULL,
    source_url       VARCHAR(256)                                       NOT NULL,
    certainty        FLOAT                                              NOT NULL
);
CREATE INDEX idx__auto_tag_session_image_option__auto_tag_session_image ON auto_tag_session_image_option (session_image_id);
