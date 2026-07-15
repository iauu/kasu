CREATE TABLE IF NOT EXISTS channel_managed
(
    channel_id TEXT PRIMARY KEY,
    config INT NOT NULL DEFAULT 0
);

/*
Bit 0, 1 (join):
0 - No restriction
1 - Restriction (bot not allowed)
2 - Restriction (invite by existing)
3 - Restriction (invite by channel manager)
Bit 2, 3(post):
0 - No restriction
1 - Restriction (bot not allowed)
2 - Restriction (invite by existing)
3 - Restriction (invite by channel manager)
Bit 4(post ext):
0 - Main channel restriction only
1 - Include thread restriction
*/

CREATE TABLE IF NOT EXISTS accepted(
    channel_id TEXT NOT NULL,
    user_id TEXT NOT NULL,
    PRIMARY KEY (channel_id, user_id),
    FOREIGN KEY (channel_id) REFERENCES channel_managed(channel_id)
);