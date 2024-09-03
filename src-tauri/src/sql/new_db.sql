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
	FOREIGN KEY("category_id") REFERENCES "category"("id") ON DELETE SET NULL,
	PRIMARY KEY("id" AUTOINCREMENT)
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
INSERT INTO "budget_plan_category" VALUES (1,1);
INSERT INTO "budget_plan_category" VALUES (2,1);
INSERT INTO "budget_plan_category" VALUES (3,1);
INSERT INTO "budget_plan_category" VALUES (4,1);
INSERT INTO "budget_plan_category" VALUES (5,1);
INSERT INTO "category" VALUES (1,'Groceries');
INSERT INTO "category" VALUES (2,'Rent');
INSERT INTO "category" VALUES (3,'Bills');
INSERT INTO "category" VALUES (4,'Gas');
INSERT INTO "category" VALUES (5,'Disposable Income');
INSERT INTO "budget_category" VALUES (1,1,'300','',0);
INSERT INTO "budget_category" VALUES (2,2,'2000','',0);
INSERT INTO "budget_category" VALUES (3,3,'400','',0);
INSERT INTO "budget_category" VALUES (4,4,'200','',0);
INSERT INTO "budget_category" VALUES (5,5,'300','',0);
INSERT INTO "budget_plan" VALUES (1,'MONTHLY',15,NULL,1,'Main Budget');
CREATE UNIQUE INDEX IF NOT EXISTS "idx_budget_plan_category" ON "budget_plan_category" (
	"budget_category_id",
	"budget_plan_id"
);
CREATE UNIQUE INDEX IF NOT EXISTS "idx_budget_budget_category" ON "budget_budget_category" (
	"budget_category_id",
	"budget_id"
);
COMMIT;
