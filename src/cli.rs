use clap::Parser;

#[derive(Parser, Debug, Clone)]
#[command(name = "sagemcom-fast-exporter")]
#[command(about = "A Prometheus exporter for Sagemcom F@st routers (lite version)", long_about = None)]
pub struct Args {
    #[arg(long, default_value = "localhost:9780")]
    pub bind: String,

    #[arg(long)]
    pub host: String,

    #[arg(long, default_value = "admin")]
    pub username: String,

    #[arg(long)]
    pub password: String,

    #[arg(long, default_value = "80")]
    pub port: u16,

    #[arg(long, default_value = "5m")]
    pub refresh_interval: String,

    #[arg(long, default_value = "MD5", value_parser = ["SHA512", "MD5"])]
    pub auth_method: String,

    #[arg(long, default_value = "info", value_parser = ["debug", "info", "warn", "error"])]
    pub log_level: String,
}

impl Args {
    pub fn refresh_interval_secs(&self) -> u64 {
        humantime::parse_duration(&self.refresh_interval)
            .map(|d| d.as_secs())
            .unwrap_or(300)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_refresh_interval_secs() {
        let args = Args {
            bind: "localhost:9780".to_string(),
            host: "192.168.2.1".to_string(),
            username: "admin".to_string(),
            password: "password".to_string(),
            port: 80,
            refresh_interval: "5m".to_string(),
            auth_method: "MD5".to_string(),
            log_level: "info".to_string(),
        };
        assert_eq!(args.refresh_interval_secs(), 300);
    }

    #[test]
    fn test_refresh_interval_secs_hours() {
        let args = Args::parse_from([
            "sagemcom-fast-exporter",
            "--host",
            "192.168.2.1",
            "--password",
            "password",
            "--refresh-interval",
            "1h",
        ]);
        assert_eq!(args.refresh_interval_secs(), 3600);
    }

    #[test]
    fn test_refresh_interval_secs_hms() {
        let args = Args::parse_from([
            "sagemcom-fast-exporter",
            "--host",
            "192.168.2.1",
            "--password",
            "password",
            "--refresh-interval",
            "1h30m0s",
        ]);
        assert_eq!(args.refresh_interval_secs(), 5400);
    }

    #[test]
    fn test_refresh_interval_secs_invalid() {
        let args = Args::parse_from([
            "sagemcom-fast-exporter",
            "--host",
            "192.168.2.1",
            "--password",
            "password",
            "--refresh-interval",
            "invalid",
        ]);
        assert_eq!(args.refresh_interval_secs(), 300);
    }
}
