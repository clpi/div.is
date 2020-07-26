-- Consider where it might be appropriate to use JSON, i.e.
-- for preferences, user info, etc. also for real/double values


CREATE SCHEMA IF NOT EXISTS UserData;

    CREATE TABLE IF NOT EXISTS UserData.Users (
        id          SERIAL NOT NULL PRIMARY KEY,
        email       TEXT NOT NULL UNIQUE,
        username    TEXT NOT NULL UNIQUE CHECK (char_length(first_name) < 40),
        password    TEXT NOT NULL CHECK (char_length(first_name) < 40),
        created_at  TIMESTAMPZ DEFAULT CURRENT_TIMESTAMP
    )

    CREATE TABLE IF NOT EXISTS UserData.UserInfo (
        id SERIAL PRIMARY KEY NOT NULL,
        uid INTEGER NOT NULL,
        timezone TIMEZONE NOT NULL,
        first_name TEXT CHECK (CHAR_LENGTH(first_name) < 80),
        last_name TEXT CHECK (CHAR_LENGTH(first_name) < 80),
        bio TEXT,
        img_path TEXT,
        gender TEXT,
        birth_date INTEGER,
        location TEXT,
        experience INTEGER NOT NULL,
        user_type INTEGER NOT NULL,
        updated_at TIMESTAMPZ DEFAULT CURRENT_TIMESTAMP,
        FOREIGN KEY (uid) REFERENCES Users(id)
    )

    CREATE TABLE IF NOT EXISTS UserData.Groups (
        id SERIAL PRIMARY KEY NOT NULL,
        name TEXT NOT NULL,
        private BOOLEAN NOT NULL DEFAULT TRUE,
        status TEXT NOT NULL,
        created_at TIMESTAMPZ DEFAULT CURRENT_TIMESTAMP
    )

    CREATE TABLE IF NOT EXISTS UserData.GroupInfo (
        id SERIAL PRIMARY KEY NOT NULL,
        description TEXT NOT NULL,
        private BOOLEAN NOT NULL DEFAULT TRUE,
        status TEXT NOT NULL,
        updated_at TIMESTAMPZ DEFAULT CURRENT_TIMESTAMP
    )

    COMMENT ON TABLE UserData.Users is "Essential user info for auth/session";
    COMMENT ON TABLE UserData.UserInfo is "Profile info for user";
    COMMENT ON TABLE UserData.UserInfo is "Groups of users";

    CREATE VIEW UserSession
        SELECT (id, email, username) FROM Users;

CREATE SCHEMA IF NOT EXISTS Objects;

    CREATE TABLE IF NOT EXISTS Objects.Records (
        id SERIAL PRIMARY KEY NOT NULL,
        uid INTEGER NOT NULL,
        name TEXT NOT NULL,    
        status TEXT NOT NULL,
        private BOOLEAN NOT NULL DEFAULT TRUE,
        created_at TIMESTAMPZ DEFAULT CURRENT_TIMESTAMP,
        FOREIGN KEY (uid) REFERENCES Users(id)
    )

    CREATE TABLE IF NOT EXISTS Objects.Items (
        id SERIAL PRIMARY KEY NOT NULL,
        uid INTEGER NOT NULL,
        name TEXT NOT NULL,
        status TEXT NOT NULL,
        private BOOLEAN NOT NULL DEFAULT TRUE,
        created_at TIMESTAMPZ DEFAULT CURRENT_TIMESTAMP,
        FOREIGN KEY (uid) REFERENCES Users(id)
    )


    CREATE TABLE IF NOT EXISTS Objects.Fields (
        id SERIAL PRIMARY KEY NOT NULL,
        name TEXT NOT NULL,    
        typ TEXT NOT NULL,
        value TEXT,
        private BOOLEAN NOT NULL DEFAULT TRUE,
        created_at TIMESTAMPZ DEFAULT CURRENT_TIMESTAMP
    )

    CREATE TABLE IF NOT EXISTS Objects.EntryTypes (
        id SERIAL PRIMARY KEY NOT NULL,
        uid INTEGER NOT NULL,
        name TEXT NOT NULL,
        private BOOLEAN NOT NULL DEFAULT TRUE,
        created_at TIMESTAMPZ DEFAULT CURRENT_TIMESTAMP,
        FOREIGN KEY (uid) REFERENCES Users(id)
    )

CREATE SCHEMA IF NOT EXISTS Entries;

    CREATE TABLE IF NOT EXISTS Entries.EntryEntries ( 
        id SERIAL PRIMARY KEY NOT NULL,
        uid INTEGER NOT NULL,
        rid INTEGER NOT NULL,
        etid INTEGER,
        created_at TIMESTAMPZ DEFAULT CURRENT_TIMESTAMP,
        FOREIGN KEY (uid) REFERENCES Users(id),
        FOREIGN KEY (rid) REFERENCES Records(id),
        FOREIGN KEY (etid) REFERENCES EntryTypes(id)
    )

    CREATE TABLE IF NOT EXISTS Entries.FieldEntries ( 
        id SERIAL PRIMARY KEY NOT NULL,
        eeid INTEGER NOT NULL,
        fid INTEGER NOT NULL,
        content TEXT,
        FOREIGN KEY (eeid) REFERENCES EntryEntries(id),
        FOREIGN KEY (fid) REFERENCES Fields(id)
    )

CREATE SCHEMA IF NOT EXISTS Logic;

    CREATE TABLE IF NOT EXISTS Logic.Rules ( 
        id SERIAL PRIMARY KEY NOT NULL,
        uid INTEGER NOT NULL,
        name TEXT NOT NULL,
        priority TEXT,
        status TEXT NOT NULL,
        created_at TIMESTAMPZ DEFAULT CURRENT_TIMESTAMP,
        FOREIGN KEY (uid) REFERENCES Users(id)
    )

    CREATE TABLE IF NOT EXISTS Logic.Conditions (
        id SERIAL PRIMARY KEY NOT NULL,
        pos INTEGER NOT NULL,
        and_or BOOLEAN,
        ruleid INTEGER NOT NULL,
        iid1 INTEGER NOT NULL,
        iid2 INTEGER NOT NULL,
        fid1 INTEGER NOT NULL,
        fid2 INTEGER NOT NULL,
        cond INTEGER NOT NULL,        
        status TEXT NOT NULL,
        created_at TIMESTAMPZ DEFAULT CURRENT_TIMESTAMP,
        FOREIGN KEY (ruleid) REFERENCES Rules(id),
        FOREIGN KEY (iid1) REFERENCES Items(id),
        FOREIGN KEY (iid2) REFERENCES Items(id),
        FOREIGN KEY (fid1) REFERENCES Fields(id),
        FOREIGN KEY (fid2) REFERENCES Fields(id)
    )

    CREATE TABLE IF NOT EXISTS Logic.Actions (
        id SERIAL PRIMARY KEY NOT NULL,
        ruleid INTEGER NOT NULL,
        target TEXT NOT NULL,
        action TEXT NOT NULL,
        created_at TIMESTAMPZ DEFAULT CURRENT_TIMESTAMP,
        FOREIGN KEY (ruleid) REFERENCES Rules(id)
    )

CREATE SCHEMA IF NOT EXISTS Links;

    CREATE TABLE IF NOT EXISTS Links.UserGroupLinks (
        id SERIAL PRIMARY KEY NOT NULL,
        uid INTEGER NOT NULL,
        gid INTEGER NOT NULL,
        role TEXT NOT NULL,
        status TEXT NOT NULL,
        created_at TIMESTAMPZ DEFAULT CURRENT_TIMESTAMP,
        FOREIGN KEY (uid) REFERENCES Users(id),
        FOREIGN KEY (gid) REFERENCES Groups(id)
    )

    CREATE TABLE IF NOT EXISTS Links.RecordItemLinks (
        id SERIAL PRIMARY KEY NOT NULL,
        rid INTEGER NOT NULL,
        iid INTEGER NOT NULL,
        status TEXT NOT NULL,
        priority TEXT,
        created_at TIMESTAMPZ DEFAULT CURRENT_TIMESTAMP,
        FOREIGN KEY (rid) REFERENCES Records(id),
        FOREIGN KEY (iid) REFERENCES Items(id)
    )

    CREATE TABLE IF NOT EXISTS Links.ItemFieldLinks (
        id SERIAL PRIMARY KEY NOT NULL,
        iid INTEGER NOT NULL,
        fid INTEGER NOT NULL,
        priority TEXT,
        created_at TIMESTAMPZ DEFAULT CURRENT_TIMESTAMP,
        FOREIGN KEY (iid) REFERENCES Items(id),
        FOREIGN KEY (fid) REFERENCES Fields(id)
    )

    CREATE TABLE IF NOT EXISTS Links.FieldEntryLinks (
        id SERIAL PRIMARY KEY NOT NULL,
        fid INTEGER NOT NULL,
        etid INTEGER NOT NULL,
        created_at TIMESTAMPZ DEFAULT CURRENT_TIMESTAMP,
        FOREIGN KEY (etid) REFERENCES EntryTypes(id),
        FOREIGN KEY (fid) REFERENCES Fields(id)
    )

    CREATE TABLE IF NOT EXISTS Links.UserRecordLinks (
        id SERIAL PRIMARY KEY NOT NULL,
        uid INTEGER NOT NULL,
        rid INTEGER NOT NULL,
        created_at TIMESTAMPZ DEFAULT CURRENT_TIMESTAMP,
        FOREIGN KEY (uid) REFERENCES Users(id),
        FOREIGN KEY (rid) REFERENCES Records(id)
    )

CREATE SCHEMA IF NOT EXISTS Relations;

    CREATE TABLE IF NOT EXISTS Relations.UserRelations (
        id SERIAL PRIMARY KEY NOT NULL,
        uid1 INTEGER NOT NULL,
        uid2 INTEGER NOT NULL,
        relation TEXT NOT NULL,
        created_at TIMESTAMPZ DEFAULT CURRENT_TIMESTAMP,
        updated_at TIMESTAMPZ DEFAULT CURRENT_TIMESTAMP,
        FOREIGN KEY (uid1) REFERENCES Users(id),
        FOREIGN KEY (uid2) REFERENCES Users(id)
    )

    CREATE TABLE IF NOT EXISTS Relations.RecordRelations (
        id SERIAL PRIMARY KEY NOT NULL,
        rid1 INTEGER NOT NULL,
        rid2 INTEGER NOT NULL,
        relation TEXT NOT NULL,
        created_at TIMESTAMPZ DEFAULT CURRENT_TIMESTAMP,
        updated_at TIMESTAMPZ DEFAULT CURRENT_TIMESTAMP,
        FOREIGN KEY (rid1) REFERENCES Records(id),
        FOREIGN KEY (rid2) REFERENCES Records(id)
    )

    CREATE TABLE IF NOT EXISTS Relations.ItemRelations (
        id SERIAL PRIMARY KEY NOT NULL,
        iid1 INTEGER NOT NULL,
        iid2 INTEGER NOT NULL,
        relation TEXT NOT NULL,
        created_at TIMESTAMPZ DEFAULT CURRENT_TIMESTAMP,
        updated_at TIMESTAMPZ DEFAULT CURRENT_TIMESTAMP,
        FOREIGN KEY (iid1) REFERENCES Items(id),
        FOREIGN KEY (iid2) REFERENCES Items(id)
    )
