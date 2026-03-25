CREATE TABLE IF NOT EXISTS task_entries (
  id TEXT PRIMARY KEY,
  date TEXT NOT NULL,
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL,
  content TEXT NOT NULL,
  status TEXT NOT NULL,
  note TEXT NOT NULL DEFAULT ''
);

CREATE TABLE IF NOT EXISTS app_settings (
  singleton_key TEXT PRIMARY KEY CHECK (singleton_key = 'default'),
  reminder_enabled INTEGER NOT NULL DEFAULT 1,
  off_work_time TEXT NOT NULL DEFAULT '18:00',
  remind_before_minutes INTEGER NOT NULL DEFAULT 5,
  global_hotkey TEXT NOT NULL DEFAULT 'Ctrl+Shift+D',
  ai_enabled INTEGER NOT NULL DEFAULT 0,
  ai_base_url TEXT NOT NULL DEFAULT '',
  ai_model TEXT NOT NULL DEFAULT '',
  ai_api_key_ref TEXT NOT NULL DEFAULT '',
  ai_system_prompt TEXT NOT NULL DEFAULT '',
  report_template TEXT NOT NULL DEFAULT 'default',
  updated_at TEXT NOT NULL
);
