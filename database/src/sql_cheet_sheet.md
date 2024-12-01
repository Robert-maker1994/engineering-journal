# SQL Cheat Sheet
| Command       | Description                                                                 |
|---------------|-----------------------------------------------------------------------------|
| `SELECT`      | Retrieves data from a database.                                             |
| `INSERT`      | Adds new data into a database table.                                        |
| `UPDATE`      | Modifies existing data in a database table.                                 |
| `DELETE`      | Removes data from a database table.                                         |
| `CREATE TABLE`| Creates a new table in the database.                                        |
| `ALTER TABLE` | Modifies an existing table structure.                                       |
| `DROP TABLE`  | Deletes a table from the database.                                          |
| `JOIN`        | Combines rows from two or more tables based on a related column.            |
| `INNER JOIN`  | Returns records that have matching values in both tables.                   |
| `LEFT JOIN`   | Returns all records from the left table, and the matched records from the right table. |
| `RIGHT JOIN`  | Returns all records from the right table, and the matched records from the left table. |
| `FULL JOIN`   | Returns all records when there is a match in either left or right table.    |
| `WHERE`       | Filters records based on a specified condition.                             |
| `GROUP BY`    | Groups rows that have the same values into summary rows.                    |
| `ORDER BY`    | Sorts the result set in ascending or descending order.                      |
| `HAVING`      | Filters records that work on summarized `GROUP BY` results.                 |
| `USING`       | Specifies which columns to test for equality when performing a join.        |

## Examples

```sql
-- SELECT example
SELECT * FROM employees;

-- INSERT example
INSERT INTO employees (name, position) VALUES ('John Doe', 'Manager');

-- UPDATE example
UPDATE employees SET position = 'Senior Manager' WHERE name = 'John Doe';

-- DELETE example
DELETE FROM employees WHERE name = 'John Doe';

-- CREATE TABLE example
CREATE TABLE employees (
    id INT PRIMARY KEY,
    name VARCHAR(100),
    position VARCHAR(100)
);

-- JOIN example
SELECT employees.name, departments.department_name
FROM employees
JOIN departments ON employees.department_id = departments.id;

-- INNER JOIN example
SELECT employees.name, departments.department_name
FROM employees
INNER JOIN departments ON employees.department_id = departments.id;

-- LEFT JOIN example
SELECT employees.name, departments.department_name
FROM employees
LEFT JOIN departments ON employees.department_id = departments.id;

-- RIGHT JOIN example
SELECT employees.name, departments.department_name
FROM employees
RIGHT JOIN departments ON employees.department_id = departments.id;

-- FULL JOIN example
SELECT employees.name, departments.department_name
FROM employees
FULL JOIN departments ON employees.department_id = departments.id;

-- USING example
SELECT employees.name, departments.department_name
FROM employees
JOIN departments USING (department_id);

-- WHERE example
SELECT * FROM employees WHERE position = 'Manager';

-- GROUP BY example 
SELECT * FROM employees
GROUP BY employees.email
HAVING COUNT(*) > 1;

-- ORDER BY example
SELECT * FROM employees ORDER BY name ASC;

-- HAVING example
SELECT position, COUNT(*) FROM employees GROUP BY position HAVING COUNT(*) > 1;
```