CREATE TABLE users (
  id uuid PRIMARY KEY,
  username varchar(50) NOT NULL,
  name text NOT NULL,
  friend_ids _uuid NOT NULL
)
