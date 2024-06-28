CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    first_name VARCHAR(50) NOT NULL,
    last_name VARCHAR(50) NOT NULL,
    email VARCHAR(50) NOT NULL UNIQUE,
    password VARCHAR(100) NOT NULL,
    is_admin BOOLEAN DEFAULT false NOT NULL, 
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TYPE todo_status AS ENUM ('pending', 'doing', 'canceled', 'done');

CREATE TABLE todos (
    id SERIAL PRIMARY KEY,
    user_id INT NOT NULL,
    title VARCHAR(50) NOT NULL,
    description TEXT,
    status todo_status DEFAULT 'pending' NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT fk_user
      FOREIGN KEY(user_id) 
        REFERENCES users(id)
);

INSERT INTO users (first_name, last_name, email, password, is_admin)
VALUES ('John', 'Doe', 'john.doe@example.com', '32$uclQZA4bN0DpisuT5mnGV2b2Zw3RYJupH/QQUrpIxvM$xECldZAK0jhtdo5vjBLzVYpTCQ8xcAFriI2oV140KDg', true);

-- 634f23224f289fe2de45dce08e4258b56ad8a1cd8e62afd327779fc5f5282450
