pub const GET_ALL_TRANSACTIONS: &str = "SELECT transaction_item.id,
       amount,
       c.id AS category_id,
       c.name AS category_name,
       transaction_date,
       transaction_item.name,
       transaction_item.recurring,
       cycle,
       day_of_month,
       day_of_week,
       transaction_item.account_id,
       credit
FROM transaction_item
JOIN category c ON c.id = transaction_item.category_id;";

pub const GET_ONE_TRANSACTION: &str = "SELECT transaction_item.id,
       amount,
       c.id AS category_id,
       c.name AS category_name,
       transaction_date,
       transaction_item.name,
       transaction_item.recurring,
       cycle,
       day_of_month,
       day_of_week,
       account_id,
       credit
FROM transaction_item
JOIN category c ON c.id = transaction_item.category_id
WHERE transaction_item.id = ?1;";

pub const ADD_TRANSACTION: &str =
"INSERT INTO transaction_item (amount, category_id, transaction_date, name,recurring,cycle,day_of_month,day_of_week,account_id,credit)
VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10);";

pub const UPDATE_TRANSACTION: &str = "UPDATE transaction_item
SET amount = ?2,
    category_id = ?3,
    transaction_date = ?4,
    name = ?5,
    recurring = ?6,
    cycle = ?7,
    day_of_month = ?8,
    day_of_week = ?9,
    account_id = ?10,
    credit = ?11
WHERE transaction_item.id = ?1;";

pub const DELETE_TRANSACTION: &str = "DELETE
FROM transaction_item
WHERE id = ?1;";

pub const INSERT_CATEGORY: &str = "INSERT INTO category (name)
VALUES (?1);";

pub const UPDATE_CATEGORY: &str = "UPDATE category
SET name = ?2
WHERE category.id = ?1;";

pub const GET_ALL_CATEGORIES: &str = "SELECT id,
       name
FROM category";

pub const GET_ONE_CATEGORY: &str = "SELECT id,
       name
FROM category
WHERE category.id = ?1;";

pub const DELETE_CATEGORY: &str = "DELETE
FROM category
WHERE id = ?1;";

pub const INSERT_ACCOUNT: &str =
"INSERT INTO account (name,created_date,currency_type)
VALUES (?1,?2,?3);";


pub const UPDATE_ACCOUNT: &str =
"UPDATE account
SET name = ?2,
       created_date = ?3,
       currency_type = ?4
WHERE id = ?1;";

pub const GET_ALL_ACCOUNTS: &str =
"SELECT id,
       name,
       created_date,
       currency_type
FROM account";

pub const GET_ONE_ACCOUNT: &str =
"SELECT id,
       name,
       created_date,
       currency_type
FROM account
WHERE id = ?1;";


pub const DELETE_ACCOUNT: &str =
"DELETE
FROM account
WHERE id = ?1;";


pub const INSERT_BUDGET_CATEGORY: &str =
    "INSERT INTO budget_category (category_id, flat_amount, percentage_amount, fixed)
VALUES (?1,?2,?3,?4);";

pub const UPDATE_BUDGET_CATEGORY: &str = "UPDATE budget_category
SET category_id = ?2,
    flat_amount = ?3,
    percentage_amount = ?4,
    fixed = ?5
WHERE id = ?1;";

pub const GET_ALL_BUDGET_CATEGORIES: &str = "SELECT id,
       category_id,
       flat_amount,
       percentage_amount,
       fixed
FROM budget_category";

pub const GET_ONE_BUDGET_CATEGORY: &str = "SELECT id,
       category_id,
       flat_amount,
       percentage_amount,
       fixed
FROM budget_category
WHERE id = ?1;";

pub const DELETE_BUDGET_CATEGORY: &str = "DELETE
FROM budget_category
WHERE id = ?1;";

pub const INSERT_BUDGET: &str = "INSERT INTO budget (start_date, CYCLE, end_date)
VALUES (?1,?2,?3);";

pub const UPDATE_BUDGET: &str = "UPDATE budget
SET start_date = ?2,
    CYCLE = ?3,
    end_date = ?4
WHERE budget.id = ?1;";

pub const GET_ALL_BUDGET: &str = "SELECT id,
       start_date,
       CYCLE,
       end_date
FROM budget";

pub const GET_ONE_BUDGET: &str = "SELECT id,
       start_date,
       CYCLE,
       end_date
FROM budget
WHERE budget.id = ?1;";

pub const DELETE_BUDGET: &str = "DELETE
FROM budget
WHERE id = ?1;";

pub const INSERT_BUDGET_PLAN: &str =
    "INSERT INTO budget_plan (CYCLE, start_date_of_month, start_date_of_week, name, active)
VALUES (?1,?2,?3,?4,?5);";

pub const UPDATE_BUDGET_PLAN: &str = "UPDATE budget_plan
SET CYCLE = ?2,
    start_date_of_month = ?3,
    start_date_of_week = ?4,
    name = ?5,
    active = ?6
WHERE budget_plan.id = ?1;";

pub const GET_ALL_BUDGET_PLAN: &str = "SELECT id,
       CYCLE,
       start_date_of_month,
       start_date_of_week,
       active,
       name
FROM budget_plan";

pub const GET_ONE_BUDGET_PLAN: &str = "SELECT id,
       CYCLE,
       start_date_of_month,
       start_date_of_week,
       active,
       name
FROM budget_plan
WHERE budget_plan.id = ?1;";

pub const DELETE_BUDGET_PLAN: &str = "DELETE
FROM budget_plan
WHERE id = ?1;";

pub const INSERT_BUDGET_BUDGET_CATEGORIES: &str =
    "INSERT INTO budget_budget_category (budget_category_id, budget_id)
VALUES (?1,?2);";

pub const GET_ALL_BUDGET_BUDGET_CATEGORIES: &str = "SELECT bc.id,
       bc.category_id,
       bc.flat_amount,
       bc.percentage_amount,
       bc.fixed,
       c.id category_id
FROM budget_budget_category bbc
JOIN budget_category bc ON bc.id = bbc.budget_category_id
JOIN category c ON c.id = bc.category_id
WHERE bbc.budget_id = ?1;";

pub const DELETE_BUDGET_BUDGET_CATEGORY: &str = "DELETE
FROM budget_budget_category
WHERE budget_category_id = ?1
  AND budget_id = ?2;";

pub const INSERT_BUDGET_PLAN_CATEGORIES: &str =
    "INSERT INTO budget_plan_category (budget_category_id, budget_plan_id)
VALUES (?1,?2);";

pub const GET_ALL_BUDGET_PLAN_CATEGORIES: &str = "SELECT bc.id,
       bc.category_id,
       bc.flat_amount,
       bc.percentage_amount,
       bc.fixed,
       c.name category_name
FROM budget_plan_category bpc
JOIN budget_category bc ON bc.id = bpc.budget_category_id
JOIN category c ON c.id = bc.category_id
WHERE bpc.budget_plan_id = ?1;";

pub const DELETE_BUDGET_PLAN_CATEGORY: &str = "DELETE
FROM budget_plan_category
WHERE budget_category_id = ?1
  AND budget_plan_id = ?2;";

pub const GET_ALL_BUDGET_STATISTICS: &str = "SELECT
      c.name as category_name,
      bc.flat_amount as category_budget,
      ( SELECT sum(ti.amount) FROM transaction_item ti
          WHERE 
          ti.category_id = c.id
          AND (ti.transaction_date BETWEEN b.start_date AND b.end_date
          OR ti.recurring)
      ) as category_spent
      FROM budget_budget_category bbc
      JOIN budget b
          ON b.id = bbc.budget_id
      JOIN budget_category bc
          ON bc.id = bbc.budget_category_id
      JOIN category c
          ON c.id = bc.category_id
      WHERE bbc.budget_id = ?1";

pub const GET_ALL_DEFAULT_BUDGET_STATISTICS: &str = "SELECT
          c.name as category_name,
          bc.flat_amount as category_budget,
          ( SELECT sum(ti.amount) FROM transaction_item ti
              WHERE 
              ti.category_id = c.id
              AND (ti.transaction_date BETWEEN ?1 AND ?2
              OR ti.recurring)
          ) as category_spent
          FROM budget_plan_category bpc
          JOIN budget_plan bp
              ON bp.id = bpc.budget_plan_id
          JOIN budget_category bc
              ON bc.id = bpc.budget_category_id
          JOIN category c
              ON c.id = bc.category_id
          WHERE bp.active = true";

pub const GET_ACTIVE_BUDGET_PLAN: &str = "SELECT id,
                 CYCLE,
                 start_date_of_month,
                 start_date_of_week,
                 active,
                 name
          FROM budget_plan
          WHERE budget_plan.active = true;";

pub const GET_ALL_TRANSACTIONS_IN_RANGE: &str = "SELECT transaction_item.id,
       amount,
       c.id AS category_id,
       c.name AS category_name,
       transaction_date,
       transaction_item.name,
       transaction_item.recurring,
       cycle,
       day_of_month,
       day_of_week,
       account_id,
       credit
FROM transaction_item
JOIN category c ON c.id = transaction_item.category_id
WHERE 
      transaction_date BETWEEN ?1 AND ?2
      OR recurring";

pub const GET_ONE_BUDGET_BY_DATE: &str = "SELECT id,
       start_date,
       CYCLE,
       end_date
FROM budget
WHERE ?1 BETWEEN budget.start_date AND budget.end_date;";

pub const GET_ACCOUNT_SUMMARY_IN_RANGE: &str = "SELECT
	a.id as account_id,
	a.name,
	(SELECT sum(ti.amount) FROM transaction_item ti
              WHERE ti.credit
              AND (ti.recurring
              OR ti.transaction_date BETWEEN ?2 AND ?3)) as credit_transactions,
	(SELECT sum(ti.amount) FROM transaction_item ti
              WHERE NOT ti.credit
              AND (ti.recurring
              OR ti.transaction_date BETWEEN ?2 AND ?3)) as debit_transactions

	FROM account a
	JOIN transaction_item ti ON ti.account_id = a.id
	WHERE a.id = ?1 AND
			  (ti.transaction_date BETWEEN ?2 AND ?3);";

pub const SQL_BUILD: &str = "BEGIN TRANSACTION;
CREATE TABLE IF NOT EXISTS \"budget_plan_category\" (
	\"budget_category_id\"	bigint NOT NULL,
	\"budget_plan_id\"	bigint NOT NULL,
	FOREIGN KEY(\"budget_category_id\") REFERENCES \"budget_category\"(\"id\") ON DELETE CASCADE,
	FOREIGN KEY(\"budget_plan_id\") REFERENCES \"budget_plan\"(\"id\") ON DELETE CASCADE
);
CREATE TABLE IF NOT EXISTS \"budget_budget_category\" (
	\"budget_category_id\"	bigint NOT NULL,
	\"budget_id\"	bigint NOT NULL,
	FOREIGN KEY(\"budget_category_id\") REFERENCES \"budget_category\"(\"id\") ON DELETE CASCADE
);
CREATE TABLE IF NOT EXISTS \"transaction_item\" (
	\"id\"	INTEGER NOT NULL,
	\"amount\"	varchar,
	\"category_id\"	bigint,
	\"transaction_date\"	DATE,
	\"name\"	varchar,
       \"recurring\" NUMERIC NOT NULL DEFAULT 0,
	FOREIGN KEY(\"category_id\") REFERENCES \"category\"(\"id\") ON DELETE SET NULL,
	PRIMARY KEY(\"id\" AUTOINCREMENT)
);
CREATE TABLE IF NOT EXISTS \"category\" (
	\"id\"	INTEGER NOT NULL UNIQUE,
	\"name\"	varchar,
	PRIMARY KEY(\"id\" AUTOINCREMENT)
);
CREATE TABLE IF NOT EXISTS \"budget_category\" (
	\"id\"	INTEGER NOT NULL UNIQUE,
	\"category_id\"	bigint,
	\"flat_amount\"	TEXT,
	\"percentage_amount\"	TEXT,
	\"fixed\"	INTEGER,
	FOREIGN KEY(\"category_id\") REFERENCES \"category\"(\"id\") ON DELETE SET NULL,
	PRIMARY KEY(\"id\" AUTOINCREMENT)
);
CREATE TABLE IF NOT EXISTS \"budget\" (
	\"id\"	bigint NOT NULL UNIQUE,
	\"start_date\"	DATE,
	\"cycle\"	varchar,
	\"end_date\"	DATE,
	PRIMARY KEY(\"id\" AUTOINCREMENT)
);
CREATE TABLE IF NOT EXISTS \"budget_plan\" (
	\"id\"	INTEGER NOT NULL UNIQUE,
	\"cycle\"	varchar,
	\"start_date_of_month\"	INTEGER,
	\"start_date_of_week\"	INTEGER,
	\"active\"	INTEGER NOT NULL DEFAULT 0,
	\"name\"	varchar NOT NULL,
	PRIMARY KEY(\"id\" AUTOINCREMENT)
);
INSERT INTO \"category\" VALUES (1,'Groceries');
INSERT INTO \"category\" VALUES (2,'Rent');
INSERT INTO \"category\" VALUES (3,'Bills');
INSERT INTO \"category\" VALUES (4,'Gas');
INSERT INTO \"category\" VALUES (5,'Disposable Income');
INSERT INTO \"budget_category\" VALUES (1,1,'300','',0);
INSERT INTO \"budget_category\" VALUES (2,2,'2000','',0);
INSERT INTO \"budget_category\" VALUES (3,3,'400','',0);
INSERT INTO \"budget_category\" VALUES (4,4,'200','',0);
INSERT INTO \"budget_category\" VALUES (5,5,'300','',0);
INSERT INTO \"budget_plan\" VALUES (1,'MONTHLY',15,NULL,1,'Main Budget');
INSERT INTO \"budget_plan_category\" VALUES (1,1);
INSERT INTO \"budget_plan_category\" VALUES (2,1);
INSERT INTO \"budget_plan_category\" VALUES (3,1);
INSERT INTO \"budget_plan_category\" VALUES (4,1);
INSERT INTO \"budget_plan_category\" VALUES (5,1);
CREATE UNIQUE INDEX IF NOT EXISTS \"idx_budget_plan_category\" ON \"budget_plan_category\" (
	\"budget_category_id\",
	\"budget_plan_id\"
);
CREATE UNIQUE INDEX IF NOT EXISTS \"idx_budget_budget_category\" ON \"budget_budget_category\" (
	\"budget_category_id\",
	\"budget_id\"
);
COMMIT;
";
