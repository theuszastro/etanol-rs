--Create Table
CREATE TABLE IF NOT EXISTS "User" (
   "id" TEXT NOT NULL PRIMARY KEY,
   "name" TEXT NOT NULL,
   "age" INTEGER,
   "isAdmin" BOOLEAN
);
