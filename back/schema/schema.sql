CREATE TABLE IF NOT EXISTS Users (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    email TEXT NOT NULL UNIQUE,
    username TEXT NOT NULL UNIQUE,
    password TEXT NOT NULL,
    created_at INTEGER NOT NULL
);

CREATE TABLE IF NOT EXISTS UserInfo (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    uid INTEGER NOT NULL,
    first_name TEXT,
    last_name TEXT,
    website TEXT,
    bio TEXT,
    img_path TEXT,
    gender TEXT,
    birth_date INTEGER,
    experience INTEGER NOT NULL,
    privelege_level INTEGER NOT NULL,
    created_at INTEGER NOT NULL,
    FOREIGN KEY (uid) REFERENCES Users(id)
);

CREATE TABLE IF NOT EXISTS Records (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    uid INTEGER NOT NULL,
    name TEXT NOT NULL,    
    status INTEGER NOT NULL DEFAULT 1,
    private BOOLEAN NOT NULL DEFAULT TRUE,
    created_at INTEGER NOT NULL,
    FOREIGN KEY (uid) REFERENCES Users(id)
);

CREATE TABLE IF NOT EXISTS Items (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    uid INTEGER NOT NULL,
    pid INTEGER,
    name TEXT NOT NULL,
    status INTEGER NOT NULL DEFAULT 1,
    private BOOLEAN NOT NULL DEFAULT TRUE,
    created_at INTEGER NOT NULL,
    FOREIGN KEY (uid) REFERENCES Users(id),
    FOREIGN KEY (pid) REFERENCES Items(id)
);

CREATE TABLE IF NOT EXISTS RecordItemLinks (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    rid INTEGER NOT NULL,
    iid INTEGER NOT NULL,
    status INTEGER NOT NULL DEFAULT 1,
    priority INTEGER DEFAULT 0,
    created_at INTEGER NOT NULL,
    FOREIGN KEY (rid) REFERENCES Records(id),
    FOREIGN KEY (iid) REFERENCES Items(id)
);

CREATE TABLE IF NOT EXISTS ItemFieldLinks (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    iid INTEGER NOT NULL,
    fid INTEGER NOT NULL,
    status INTEGER NOT NULL DEFAULT 1,
    priority INTEGER DEFAULT 0,
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
    name TEXT NOT NULL,    
    typ TEXT NOT NULL,
    value TEXT,
    private BOOLEAN NOT NULL DEFAULT TRUE,
    created_at INTEGER NOT NULL
);

CREATE TABLE IF NOT EXISTS EntryTypes (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    uid INTEGER NOT NULL,
    name TEXT NOT NULL,
    status INTEGER NOT NULL DEFAULT 1,
    private BOOLEAN NOT NULL DEFAULT TRUE,
    created_at INTEGER NOT NULL,
    FOREIGN KEY (uid) REFERENCES Users(id)
);

CREATE TABLE IF NOT EXISTS EntryEntries ( 
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    uid INTEGER NOT NULL,
    etid INTEGER,
    created_at INTEGER NOT NULL,
    FOREIGN KEY (uid) REFERENCES Users(id),
    FOREIGN KEY (etid) REFERENCES EntryTypes(id)
);

CREATE TABLE IF NOT EXISTS FieldEntries ( 
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    eeid INTEGER NOT NULL,
    fid INTEGER NOT NULL,
    content TEXT,
    FOREIGN KEY (eeid) REFERENCES EntryEntries(id),
    FOREIGN KEY (fid) REFERENCES Fields(id)
);

CREATE TABLE IF NOT EXISTS Rules ( 
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    uid INTEGER NOT NULL,
    name TEXT NOT NULL,
    priority INTEGER DEFAULT 0,
    status INTEGER NOT NULL DEFAULT 1,
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
    status INTEGER NOT NULL DEFAULT 1,
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
    target TEXT NOT NULL,
    action TEXT NOT NULL,
    created_at INTEGER NOT NULL,
    FOREIGN KEY (ruleid) REFERENCES Rules(id)
);

CREATE TABLE IF NOT EXISTS Groups (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    private BOOLEAN NOT NULL DEFAULT TRUE,
    status INTEGER NOT NULL DEFAULT 1,
    created_at INTEGER NOT NULL
);

CREATE TABLE IF NOT EXISTS UserGroupLinks (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    uid INTEGER NOT NULL,
    gid INTEGER NOT NULL,
    role TEXT NOT NULL,
    status INTEGER NOT NULL DEFAULT 1,
    created_at INTEGER NOT NULL,
    FOREIGN KEY (uid) REFERENCES Users(id),
    FOREIGN KEY (gid) REFERENCES Groups(id)
);
