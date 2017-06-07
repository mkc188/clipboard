-- Your SQL goes here
CREATE TABLE tunnels (
  id INTEGER AUTO_INCREMENT PRIMARY KEY,
  mobile_id VARCHAR(255) NOT NULL,
  computer_id VARCHAR(255) NOT NULL,
  created_time INTEGER NOT NULL
);