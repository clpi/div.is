 -- TABLE Users; (id ...)
 -- TABLE Records; (id ...)
 -- TABLE Items; (id ...)
 -- TABLE Fields; (id ...)
 -- TABLE EntryTypes; (id ..)
 -- TABLE RecordItemLinks; (id, rid, iid)
 -- TABLE ItemFieldLinks; (id, iid, fid)
 -- TABLE FieldEntryLinks; id, fid, eid)
 -- TABLE UserGroupLinks; (id, uid, gid)
 -- TABLE UserRecordLinks; (id, uid, rid)

-- Get all records associated with a single user (with id)
SELECT * FROM Records r
    JOIN UserRecordLinks url ON (r.id = url.rid)
    JOIN Users u ON (u.id = url.uid)
    WHERE u.id = ?

-- GET ALL users associated with a single record (with id)
SELECT * FROM Users u
    JOIN UserRecordLinks url ON (u.id = url.uid)
    JOIN Records r ON (r.id = url.rid)
    WHERE r.id = ?

-- GET ALL users associated with any record that is associated with an item
SELECT * FROM Users u
    JOIN UserRecordLinks url ON (u.id = url.uid)
    JOIN Records r ON (r.id = url.rid)
    WHERE r.id IN 
    (SELECT id FROM Records r1
        JOIN RecordItemLinks ril ON (r1.id = ril.rid)
        JOIN Items i on (i.id = ril.iid)
        WHERE i.id = ?)
