SELECT
  id, name, age
FROM users;

SELECT
  id, name, age
FROM users
ORDER_BY user ASC;

SELECT
  id, name, age
FROM users
WHERE age < 30
  AND active = true
ORDER_BY id DESC
LIMIT 3;