pub const GET_ALL_TRANSACTIONS: &str =
"SELECT transaction_item.id,
       amount,
       c.id AS category_id,
       c.name AS category_name,
       transaction_date,
       transaction_item.name
FROM transaction_item
JOIN category c ON c.id = transaction_item.category_id;";


pub const GET_ONE_TRANSACTION: &str =
"SELECT transaction_item.id,
       amount,
       c.id AS category_id,
       c.name AS category_name,
       transaction_date,
       transaction_item.name
FROM transaction_item
JOIN category c ON c.id = transaction_item.category_id
WHERE id = ?1;";


pub const ADD_TRANSACTION: &str =
"INSERT INTO transaction_item (amount, category_id, transaction_date, name)
VALUES (?1,?2,?3,?4);";


pub const UPDATE_TRANSACTION: &str =
"UPDATE transaction_item
SET amount = ?2,
    category_id = ?3,
    transaction_date = ?4,
    name = ?5
WHERE transaction_item.id = ?1;";

pub const DELETE_TRANSACTION: &str =
"DELETE
FROM transaction_item
WHERE id = ?1;";


pub const INSERT_CATEGORY: &str =
"INSERT INTO category (name)
VALUES (?1);";


pub const UPDATE_CATEGORY: &str =
"UPDATE category
SET name = ?2
WHERE category.id = ?1;";

pub const GET_ALL_CATEGORIES: &str =
"SELECT id,
       name
FROM category";

pub const GET_ONE_CATEGORY: &str =
"SELECT id,
       name
FROM category
WHERE category.id = ?1;";


pub const DELETE_CATEGORY: &str =
"DELETE
FROM category
WHERE id = ?1;";


pub const INSERT_BUDGET_CATEGORY: &str =
"INSERT INTO budget_category (category_id, flat_amount, percentage_amount, fixed)
VALUES (?1,?2,?3,?4);";


pub const UPDATE_BUDGET_CATEGORY: &str =
"UPDATE budget_category
SET category_id = ?2,
    flat_amount = ?3,
    percentage_amount = ?4,
    fixed = ?5
WHERE category.id = ?1;";


pub const GET_ALL_BUDGET_CATEGORIES: &str =
"SELECT id,
       category_id,
       flat_amount,
       percentage_amount,
       fixed
FROM budget_category";

pub const GET_ONE_BUDGET_CATEGORY: &str =
"SELECT id,
       category_id,
       flat_amount,
       percentage_amount,
       fixed
FROM budget_category
WHERE category.id = ?1;";


pub const DELETE_BUDGET_CATEGORY: &str =
"DELETE
FROM budget_category
WHERE id = ?1;";


pub const INSERT_BUDGET: &str =
"INSERT INTO budget (start_date, CYCLE, end_date)
VALUES (?1,?2,?3);";


pub const UPDATE_BUDGET: &str =
"UPDATE budget
SET start_date = ?2,
    CYCLE = ?3,
    end_date = ?3
WHERE budget.id = ?1;";

pub const GET_ALL_BUDGET: &str =
"SELECT id,
       start_date,
       CYCLE,
       end_date
FROM budget";

pub const GET_ONE_BUDGET: &str =
"SELECT id,
       start_date,
       CYCLE,
       end_date
FROM budget
WHERE budget.id = ?1;";


pub const DELETE_BUDGET: &str =
"DELETE
FROM budget
WHERE id = ?1;";


pub const INSERT_BUDGET_PLAN: &str =
"INSERT INTO budget_plan (CYCLE, start_date_of_month, start_date_of_week, name, active)
VALUES (?1,?2,?3,?4,?5);";


pub const UPDATE_BUDGET_PLAN: &str =
"UPDATE budget_plan
SET CYCLE = ?2,
    start_date_of_month = ?3,
    start_date_of_week = ?4,
    name = ?5,
    active = ?6
WHERE budget_plan.id = ?1;";


pub const GET_ALL_BUDGET_PLAN: &str =
"SELECT id,
       CYCLE,
       start_date_of_month,
       start_date_of_week,
       active,
       name
FROM budget_plan";

pub const GET_ONE_BUDGET_PLAN: &str =
"SELECT id,
       CYCLE,
       start_date_of_month,
       start_date_of_week,
       active,
       name
FROM budget_plan
WHERE budget_plan.id = ?1;";


pub const DELETE_BUDGET_PLAN: &str =
"DELETE
FROM budget_plan
WHERE id = ?1;";


pub const INSERT_BUDGET_BUDGET_CATEGORIES: &str =
"INSERT INTO budget_budget_category (budget_category_id, budget_id)
VALUES (?1,?2);";

pub const GET_ALL_BUDGET_BUDGET_CATEGORIES: &str =
"SELECT bc.id,
       bc.category_id,
       bc.flat_amount,
       bc.percentage_amount,
       bc.fixed,
       c.id category_id
FROM budget_budget_category bbc
JOIN budget_category bc ON bc.id = bbc.budget_category_id
JOIN category c ON c.id = bc.category_id
WHERE bbc.budget_id = ?1;";


pub const DELETE_BUDGET_BUDGET_CATEGORY: &str =
"DELETE
FROM budget_budget_category
WHERE budget_category_id = ?1
  AND budget_id = ?2;";


pub const INSERT_BUDGET_PLAN_CATEGORIES: &str =
"INSERT INTO budget_plan_category (budget_category_id, budget_plan_id)
VALUES (?1,?2);";


pub const GET_ALL_BUDGET_PLAN_CATEGORIES: &str =
"SELECT bc.id,
       bc.category_id,
       bc.flat_amount,
       bc.percentage_amount,
       bc.fixed,
       c.name category_name
FROM budget_plan_category bpc
JOIN budget_category bc ON bc.id = bpc.budget_category_id
JOIN category c ON c.id = bc.category_id
WHERE bpc.budget_plan_id = ?1;";


pub const DELETE_BUDGET_PLAN_CATEGORY: &str =
"DELETE
FROM budget_plan_category
WHERE budget_category_id = ?1
  AND budget_plan_id = ?2;";

pub const GET_ALL_BUDGET_STATISTICS: &str = 
  "SELECT
      c.name as category_name,
      bc.flat_amount as category_budget,
      ( SELECT sum(ti.amount) FROM transaction_item ti
          WHERE 
          ti.category_id = c.id
          AND ti.transaction_date BETWEEN b.start_date AND b.end_date
      ) as category_spent
      FROM budget_budget_category bbc
      JOIN budget b
          ON b.id = bbc.budget_id
      JOIN budget_category bc
          ON bc.id = bbc.budget_category_id
      JOIN category c
          ON c.id = bc.category_id
      WHERE bbc.budget_id = ?1";
  
pub const GET_ALL_TRANSACTIONS_IN_RANGE: &str = 
  "SELECT * FROM
      transaction_item
  WHERE 
      transaction_date BETWEEN ?1 AND ?2";