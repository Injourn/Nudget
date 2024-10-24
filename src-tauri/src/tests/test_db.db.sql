BEGIN TRANSACTION;
CREATE TABLE IF NOT EXISTS "category" (
	"id"	INTEGER NOT NULL UNIQUE,
	"name"	varchar,
	PRIMARY KEY("id" AUTOINCREMENT)
);
CREATE TABLE "account" (
	"id"	INTEGER NOT NULL,
	"name"	TEXT NOT NULL,
	"created_date"	TEXT NOT NULL,
	"currency_type"	TEXT,
	PRIMARY KEY("id" AUTOINCREMENT)
);
CREATE TABLE IF NOT EXISTS "budget" (
	"id"	INTEGER NOT NULL UNIQUE,
	"start_date"	DATE,
	"cycle"	varchar,
	"end_date"	DATE,
	PRIMARY KEY("id" AUTOINCREMENT)
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
CREATE TABLE IF NOT EXISTS "budget_category" (
	"id"	INTEGER NOT NULL UNIQUE,
	"category_id"	bigint,
	"flat_amount"	TEXT,
	"percentage_amount"	TEXT,
	"fixed"	INTEGER,
	PRIMARY KEY("id" AUTOINCREMENT),
	FOREIGN KEY("category_id") REFERENCES "category"("id") ON DELETE SET NULL
);
CREATE TABLE IF NOT EXISTS "budget_plan_category" (
	"budget_category_id"	bigint NOT NULL,
	"budget_plan_id"	bigint NOT NULL,
	FOREIGN KEY("budget_plan_id") REFERENCES "budget_plan"("id") ON DELETE CASCADE,
	FOREIGN KEY("budget_category_id") REFERENCES "budget_category"("id") ON DELETE CASCADE
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
	"recurring"	NUMERIC NOT NULL DEFAULT 0,
	"cycle"	TEXT,
	"day_of_month"	INTEGER,
	"day_of_week"	INTEGER,
	PRIMARY KEY("id" AUTOINCREMENT),
	FOREIGN KEY("category_id") REFERENCES "category"("id") ON DELETE SET NULL
);
INSERT INTO "category" ("id","name") VALUES (2,'Groceries'),
 (3,'Rent'),
 (5,'Bills'),
 (6,'Gas'),
 (7,'Disposable Income');
INSERT INTO "account" ("id","name","created_date","currency_type") VALUES (1,"initial","2024-10-15","USD"),
(2,"secondary","2024-10-20","USD");
INSERT INTO "budget" ("id","start_date","cycle","end_date") VALUES (1,'2024-06-15','MONTHLY','2024-07-14'),
(2,'2025-06-15','MONTHLY','2025-07-14');
INSERT INTO "budget_plan" ("id","cycle","start_date_of_month","start_date_of_week","active","name") VALUES (2,'MONTHLY',15,NULL,1,'Main Budget'),
 (3,'WEEKLY',NULL,3,0,'new budget');
INSERT INTO "budget_category" ("id","category_id","flat_amount","percentage_amount","fixed") VALUES (2,2,'1000','',0),
 (3,3,'800','',0),
 (14,5,'200','',0),
 (20,5,'1800','',0),
 (34,3,'1409','',0),
 (36,5,'200','',0),
 (37,2,'350','',0),
 (38,7,'600','',0),
 (39,6,'175','',0);
INSERT INTO "budget_plan_category" ("budget_category_id","budget_plan_id") VALUES (20,3),
 (34,2),
 (36,2),
 (37,2),
 (38,2),
 (39,2);
INSERT INTO "budget_budget_category" ("budget_category_id","budget_id") VALUES (2,1),
 (3,1),
 (14,1);
INSERT INTO "transaction_item" ("id","amount","category_id","transaction_date","name","recurring","cycle","day_of_month","day_of_week") VALUES (11,'150',2,'2024-06-03','Turkey sandwhich',0,NULL,NULL,NULL),
 (12,'12.43',3,'2024-06-03','test',0,NULL,NULL,NULL),
 (14,'105.5',5,'2024-06-05','Internet bill',0,NULL,NULL,NULL),
 (15,'55.53',2,'2024-06-05','Food market',0,NULL,NULL,NULL),
 (16,'30',6,'2024-06-10','Shell',0,NULL,NULL,NULL),
 (17,'200',2,'2024-07-18','Raw Cow''s foot',0,NULL,NULL,NULL),
 (18,'1409',3,'2024-08-01','Rent',0,NULL,NULL,NULL),
 (19,'30',6,'2024-07-20','Barg n'' mart',0,NULL,NULL,NULL),
 (20,'30.25',6,'2024-07-27','Barg n'' mart',0,NULL,NULL,NULL),
 (21,'1409',3,'2024-09-01','Rent',1,'MONTHLY',1,NULL),
 (22,'105.26',5,'2024-09-03','Internet bill',1,'MONTHLY',3,NULL),
 (23,'30',6,'2024-09-03','Bark n'' mart',0,NULL,NULL,NULL),
 (28,'30',2,'2024-09-03','Super store',0,NULL,NULL,NULL),
 (29,'27.17',2,'2024-09-05','Food market',0,NULL,NULL,NULL),
 (30,'800',7,'2024-09-05','Barbados',0,NULL,NULL,NULL),
 (32,'33.86',2,'2024-09-17','Super store',0,NULL,NULL,NULL),
 (33,'5.2',2,'2024-09-17','Water',0,NULL,NULL,NULL),
 (34,'81.87',7,'2024-09-20','Frederico''s',0,NULL,NULL,NULL),
 (35,'15.37',7,'2024-09-21','Milk and Shake',0,NULL,NULL,NULL),
 (36,'26.36',6,'2024-09-21','Barg n'' mart',0,NULL,NULL,NULL),
 (37,'55.64',2,'2024-09-21','Super store',0,NULL,NULL,NULL),
 (38,'2.6',2,'2024-09-26','water',0,NULL,NULL,NULL),
 (39,'72.34',2,'2024-09-26','Food market',0,NULL,NULL,NULL);
CREATE UNIQUE INDEX IF NOT EXISTS "idx_budget_plan_category" ON "budget_plan_category" (
	"budget_category_id",
	"budget_plan_id"
);
CREATE UNIQUE INDEX IF NOT EXISTS "idx_budget_budget_category" ON "budget_budget_category" (
	"budget_category_id",
	"budget_id"
);
COMMIT;
