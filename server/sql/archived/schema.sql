-- $07/25/20$ ARCHIVED: Now using Postgres --

CREATE TABLE IF NOT EXISTS Users (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    email VARCHAR(255) NOT NULL UNIQUE,
    username VARCHAR(64) NOT NULL UNIQUE,
    password VARCHAR(255) NOT NULL,
    created_at INTEGER NOT NULL
);

CREATE TABLE IF NOT EXISTS UserInfo (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    uid INTEGER NOT NULL,
    first_name VARCHAR(255),
    last_name VARCHAR(255),
    bio VARCHAR(255),
    img_path VARCHAR(255),
    gender VARCHAR(255),
    birth_date INTEGER,
    location VARCHAR(255),
    experience INTEGER NOT NULL,
    user_type INTEGER NOT NULL,
    created_at INTEGER NOT NULL,
    FOREIGN KEY (uid) REFERENCES Users(id)
);

CREATE TABLE IF NOT EXISTS Records (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    uid INTEGER NOT NULL,
    name VARCHAR(255) NOT NULL,    
    status VARCHAR(255) NOT NULL,
    private BOOLEAN NOT NULL DEFAULT TRUE,
    created_at INTEGER NOT NULL,
    FOREIGN KEY (uid) REFERENCES Users(id)
);

CREATE TABLE IF NOT EXISTS Items (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    uid INTEGER NOT NULL,
    name VARCHAR(255) NOT NULL,
    status VARCHAR(255) NOT NULL,
    private BOOLEAN NOT NULL DEFAULT TRUE,
    created_at INTEGER NOT NULL,
    FOREIGN KEY (uid) REFERENCES Users(id),
    FOREIGN KEY (pid) REFERENCES Items(id)
);

CREATE TABLE IF NOT EXISTS RecordItemLinks (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    rid INTEGER NOT NULL,
    iid INTEGER NOT NULL,
    status VARCHAR(255) NOT NULL,
    priority VARCHAR(255),
    created_at INTEGER NOT NULL,
    FOREIGN KEY (rid) REFERENCES Records(id),
    FOREIGN KEY (iid) REFERENCES Items(id)
);

CREATE TABLE IF NOT EXISTS ItemFieldLinks (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    iid INTEGER NOT NULL,
    fid INTEGER NOT NULL,
    priority VARCHAR(255),
    created_at INTEGER NOT NULL,
    FOREIGN KEY (iid) REFERENCES Items(id),
    FOREIGN KEY (fid) REFERENCES Fields(id)
);

CREATE TABLE IF NOT EXISTS FieldEntryLinks (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    fid INTEGER NOT NULL,
    etid INTEGER NOT NULL,
    created_at INTEGER NOT NULL,
    FOREIGN KEY (etid) REFERENCES EntryTypes(id),
    FOREIGN KEY (fid) REFERENCES Fields(id)
);

CREATE TABLE IF NOT EXISTS Fields (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name VARCHAR(255) NOT NULL,    
    typ VARCHAR(255) NOT NULL,
    value VARCHAR(255),
    private BOOLEAN NOT NULL DEFAULT TRUE,
    created_at INTEGER NOT NULL
);

CREATE TABLE IF NOT EXISTS EntryTypes (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    uid INTEGER NOT NULL,
    name VARCHAR(255) NOT NULL,
    private BOOLEAN NOT NULL DEFAULT TRUE,
    created_at INTEGER NOT NULL,
    FOREIGN KEY (uid) REFERENCES Users(id)
);

CREATE TABLE IF NOT EXISTS EntryEntries ( 
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    uid INTEGER NOT NULL,
    rid INTEGER NOT NULL,
    etid INTEGER,
    created_at INTEGER NOT NULL,
    FOREIGN KEY (uid) REFERENCES Users(id),
    FOREIGN KEY (rid) REFERENCES Records(id),
    FOREIGN KEY (etid) REFERENCES EntryTypes(id)
);

CREATE TABLE IF NOT EXISTS FieldEntries ( 
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    eeid INTEGER NOT NULL,
    fid INTEGER NOT NULL,
    content VARCHAR(255),
    FOREIGN KEY (eeid) REFERENCES EntryEntries(id),
    FOREIGN KEY (fid) REFERENCES Fields(id)
);

CREATE TABLE IF NOT EXISTS Rules ( 
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    uid INTEGER NOT NULL,
    name VARCHAR(255) NOT NULL,
    priority VARCHAR(255),
    status VARCHAR(255) NOT NULL,
    created_at INTEGER NOT NULL,
    FOREIGN KEY (uid) REFERENCES Users(id)
);

CREATE TABLE IF NOT EXISTS Conditions (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    pos INTEGER NOT NULL,
    and_or BOOLEAN,
    ruleid INTEGER NOT NULL,
    iid1 INTEGER NOT NULL,
    iid2 INTEGER NOT NULL,
    fid1 INTEGER NOT NULL,
    fid2 INTEGER NOT NULL,
    cond INTEGER NOT NULL,        
    status VARCHAR(255) NOT NULL,
    created_at INTEGER NOT NULL,
    FOREIGN KEY (ruleid) REFERENCES Rules(id),
    FOREIGN KEY (iid1) REFERENCES Items(id),
    FOREIGN KEY (iid2) REFERENCES Items(id),
    FOREIGN KEY (fid1) REFERENCES Fields(id),
    FOREIGN KEY (fid2) REFERENCES Fields(id)
);

CREATE TABLE IF NOT EXISTS Actions (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    ruleid INTEGER NOT NULL,
    target VARCHAR(255) NOT NULL,
    action VARCHAR(255) NOT NULL,
    created_at INTEGER NOT NULL,
    FOREIGN KEY (ruleid) REFERENCES Rules(id)
);

CREATE TABLE IF NOT EXISTS Groups (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name VARCHAR(255) NOT NULL,
    private BOOLEAN NOT NULL DEFAULT TRUE,
    status VARCHAR(255) NOT NULL,
    created_at INTEGER NOT NULL
);

CREATE TABLE IF NOT EXISTS UserGroupLinks (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    uid INTEGER NOT NULL,
    gid INTEGER NOT NULL,
    role VARCHAR(255) NOT NULL,
    status VARCHAR(255) NOT NULL,
    created_at INTEGER NOT NULL,
    FOREIGN KEY (uid) REFERENCES Users(id),
    FOREIGN KEY (gid) REFERENCES Groups(id)
);

CREATE TABLE IF NOT EXISTS UserRecordLinks (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    uid INTEGER NOT NULL,
    rid INTEGER NOT NULL,
    created_at INTEGER NOT NULL,
    FOREIGN KEY (uid) REFERENCES Users(id),
    FOREIGN KEY (rid) REFERENCES Records(id)
);

CREATE TABLE IF NOT EXISTS UserRelationships (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    uid1 INTEGER NOT NULL,
    uid2 INTEGER NOT NULL,
    relation VARCHAR(255) NOT NULL,
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL,
    FOREIGN KEY (uid1) REFERENCES Users(id),
    FOREIGN KEY (uid2) REFERENCES Users(id)
);

CREATE TABLE IF NOT EXISTS RecordRelationships (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    rid1 INTEGER NOT NULL,
    rid2 INTEGER NOT NULL,
    relation VARCHAR(255) NOT NULL,
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL,
    FOREIGN KEY (rid1) REFERENCES Records(id),
    FOREIGN KEY (rid2) REFERENCES Records(id)
);

CREATE TABLE IF NOT EXISTS ItemRelationships (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    iid1 INTEGER NOT NULL,
    iid2 INTEGER NOT NULL,
    relation VARCHAR(255) NOT NULL,
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL,
    FOREIGN KEY (iid1) REFERENCES Items(id),
    FOREIGN KEY (iid2) REFERENCES Items(id)
);
