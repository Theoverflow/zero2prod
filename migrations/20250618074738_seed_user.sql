-- Add migration script here
INSERT INTO users (user_id, username, password_hash)
VALUES (
    'ddf8994f-d522-4659-8d02-c1d479057be6',
    'admin',
    '$argon2id$v=19$m=15000,t=2,p=1$ycwZSACbff1O4Qtj3JIV5g$9xnbkcV24bWzw2dN2SfLx/DRNe/oCZcMqxF2NaYfhE0'
);