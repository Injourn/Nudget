BEGIN TRANSACTION;
CREATE TABLE IF NOT EXISTS "budget_plan_category" (
	"budget_category_id"	bigint NOT NULL,
	"budget_plan_id"	bigint NOT NULL,
	FOREIGN KEY("budget_category_id") REFERENCES "budget_category"("id") ON DELETE CASCADE,
	FOREIGN KEY("budget_plan_id") REFERENCES "budget_plan"("id") ON DELETE CASCADE
);
CREATE TABLE IF NOT EXISTS "budget_budget_category" (
	"budget_category_id"	bigint NOT NULL,
	"budget_id"	bigint NOT NULL,
	FOREIGN KEY("budget_category_id") REFERENCES "budget_category"("id") ON DELETE CASCADE
);
CREATE TABLE IF NOT EXISTS "transaction_item" (
	"id"	INTEGER NOT NULL,
	"amount"	varchar,
	"category_id"	bigint,
	"transaction_date"	DATE,
	"name"	varchar,
	FOREIGN KEY("category_id") REFERENCES "category"("id") ON DELETE SET NULL,
	PRIMARY KEY("id" AUTOINCREMENT)
);
CREATE TABLE IF NOT EXISTS "category" (
	"id"	INTEGER NOT NULL UNIQUE,
	"name"	varchar,
	PRIMARY KEY("id" AUTOINCREMENT)
);
CREATE TABLE IF NOT EXISTS "budget_category" (
	"id"	INTEGER NOT NULL UNIQUE,
	"category_id"	bigint,
	"flat_amount"	TEXT,
	"percentage_amount"	TEXT,
	"fixed"	INTEGER,
	PRIMARY KEY("id" AUTOINCREMENT),
	FOREIGN KEY("category_id") REFERENCES "category"("id") ON DELETE SET NULL
);
CREATE TABLE IF NOT EXISTS "budget" (
	"id"	bigint auto_increment NOT NULL UNIQUE,
	"start_date"	DATE,
	"cycle"	varchar,
	"end_date"	DATE,
	PRIMARY KEY("id")
);
CREATE TABLE IF NOT EXISTS "budget_plan" (
	"id"	INTEGER NOT NULL UNIQUE,
	"cycle"	varchar,
	"start_date_of_month"	INTEGER,
	"start_date_of_week"	INTEGER,
	"active"	INTEGER NOT NULL DEFAULT 0,
	"name"	varchar NOT NULL,
	PRIMARY KEY("id" AUTOINCREMENT)
);
CREATE UNIQUE INDEX IF NOT EXISTS "idx_budget_plan_category" ON "budget_plan_category" (
	"budget_category_id",
	"budget_plan_id"
);
CREATE UNIQUE INDEX IF NOT EXISTS "idx_budget_budget_category" ON "budget_budget_category" (
	"budget_category_id",
	"budget_id"
);
COMMIT;
