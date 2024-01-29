# Scrapbook Backend

## Resource

1. [BSON](https://bsonspec.org/)
2. [bitcask](https://github.com/basho/bitcask)

## 

### Compaction & Merge

1. [SQLite - VACUUM](https://www.sqlite.org/lang_vacuum.html)
2. [Wiredtiger - Compaction](https://source.wiredtiger.com/develop/arch-compact.html)

In SQLite, VACUUM command rebuilds the database file, repacking it into a minimal amount of disk space.

The VACUUM command works by copying the contents of the database into a temporary database file and then overwriting the original with the contents of the temporary file. 
When overwriting the original, a rollback journal or write-ahead log WAL file is used just as it would be for any other database transaction. 
This means that when VACUUMing a database, as much as twice the size of the original database file is required in free disk space.

In Wiredtiger, Compaction is a cooperative process between the Btree layer and the Block Manager to reduce the on-disk footprint of WT tables.
