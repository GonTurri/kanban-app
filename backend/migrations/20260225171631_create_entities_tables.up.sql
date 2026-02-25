CREATE TABLE boards (
    id UUID PRIMARY KEY,
    owner_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE ,
    title VARCHAR(255) NOT NULL,
    description TEXT
);

CREATE TYPE board_role AS ENUM('owner', 'editor', 'viewer');

CREATE TABLE board_members (
  id UUID PRIMARY KEY,
  user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
  board_id UUID REFERENCES boards(id) ON DELETE CASCADE,
  role board_role NOT NULL,
  UNIQUE (board_id, user_id)
);

CREATE TYPE column_kind AS ENUM('todo', 'wip', 'done');

CREATE TABLE board_columns (
    id UUID PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    order_index BIGINT NOT NULL CHECK (order_index >= 0),
    type column_kind NOT NULL,
    column_limit INTEGER CHECK (column_limit >= 0),
    board_id UUID REFERENCES boards(id) ON DELETE CASCADE
);

CREATE TYPE item_priority AS ENUM('low', 'medium', 'high');

CREATE TABLE board_items (
    id UUID PRIMARY KEY,
    title VARCHAR(255) NOT NULL,
    description TEXT,
    priority item_priority NOT NULL,
    done BOOLEAN NOT NULL,
    column_id UUID NOT NULL REFERENCES board_columns(id) ON DELETE CASCADE,
    assigned_to UUID REFERENCES users(id) ON DELETE SET NULL,
    board_id UUID NOT NULL REFERENCES boards(id) ON DELETE CASCADE,
    created_at TIMESTAMPTZ DEFAULT now() NOT NULL
);

CREATE TABLE item_histories (
    id UUID PRIMARY KEY,
    timestamp TIMESTAMPTZ DEFAULT now() NOT NULL,
    prev_column_id UUID REFERENCES board_columns(id) ON DELETE CASCADE,
    new_column_id UUID NOT NULL REFERENCES board_columns(id) ON DELETE CASCADE,
    item_id UUID NOT NULL REFERENCES board_items(id) ON DELETE CASCADE
);