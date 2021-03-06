use serde_json;

use error::*;
use Proxer;
use models::*;

/// Gibt die neuesten News aus.
/// Der Bildlink einer News setzt sich zusammen aus: cdn.proxer.me/news/[News-ID]_[Image-ID].png
/// Für Tumbnail: cdn.proxer.me/news/th/[News-ID]_[Image-ID].png
/// Link zum Forumspost der News: proxer.me/forum/[catid]/[mid]
#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct News {
    /// Die ID der News
    pub nid: u64,
    /// Der Zeitpunkt der publizierung (Unix-Timestamp als Sekunden gespeichert)
    pub time: i64,
    /// Die ID des entsprechenden Forumsbeitrags
    pub mid: u64,
    /// Die Beschreibung der News
    pub description: String,
    /// ID zum Bild.
    pub image_id: String,
    /// CSS-Konforme Style-Elemente um die Positionierung des Bildes zu bestimmen.
    pub image_style: String,
    /// Der Titel des entsprechenden Forumsbeitrags
    pub subject: String,
    /// Anzahl der Zugriffe auf den entsprechenden Forumsbeitrag
    pub hits: u64,
    /// mid
    pub thread: u64,
    /// User-ID des Erstellers des Forumsposts
    pub uid: u64,
    /// Benutzername des Autors
    pub uname: String,
    /// Anzahl der Antworten/Kommentare auf die News
    pub posts: u64,
    /// Die ID der Kategorie, in der sich eine News befindet.
    pub catid: u64,
    /// Der Name der Kategorie.
    pub catname: String,
}

/// Diese Klasse beinhaltet alle Schnittstellen,
/// die mit Daten zu tun haben, die normalerweise auf Proxer oben rechts bei den Notifications zu sehen sind,
/// insbesondere News und Benachrichtigungen.
#[derive(Debug)]
pub struct Notification<'a> {
    proxer: &'a Proxer,
}

impl<'a> Notification<'a> {
    #[doc(hidden)]
    pub fn new(proxer: &'a Proxer) -> Notification<'a> {
        Notification { proxer: proxer }
    }

    /// Diese Funktion gibt die Anzahl an verschiedenen Notification-Kategorien zurück (kleine rote Zahlen auf Proxer).
    /// Die Ausgabedaten befinden sich in einem Array, wobei folgende Indices folgende Anzahlen enthalten:
    ///
    /// * 0 = Error (entspricht error-Wert des jsons)
    /// * 1 = Altes PN-System (deprecated)
    /// * 2 = Neues PN-System
    /// * 3 = Freundschaftsanfragen
    /// * 4 = News
    /// * 5 = Benachrichtigungen
    pub fn get_count(&self) -> Result<String> {
        let url = url!("notifications", "count");
        let body = String::new();
        let response = self.proxer.connect(&url, &body)?;
        let data: Response<String> = serde_json::from_reader(response)?;
        check_error!(data.error, data.code.unwrap_or_default(), data.message);
        check_data!(data.data)
    }

    /// Gibt die neuesten News aus.
    /// Der Bildlink einer News setzt sich zusammen aus: cdn.proxer.me/news/[News-ID]_[Image-ID].png
    /// Für Tumbnail: cdn.proxer.me/news/th/[News-ID]_[Image-ID].png
    /// Link zum Forumspost der News: proxer.me/forum/[catid]/[mid]
    ///
    /// # Arguments
    ///
    /// * `page` - Die zu ladende Seite, beginnend ab 0 (Auf Seite 0 befinden sich die neuesten News,
    /// nach hinten werden die News älter). Wenn nicht gegeben, so wird die erste Seite geladen.
    /// * `limit` - Die Anzahl der zu ladenden News pro Seite. Default 15.
    pub fn get_news_per_api(&self, page: Option<u64>, limit: Option<u64>) -> Result<Vec<News>> {
        let url = url!("notifications", "news");
        let body = param_build!("p" => page, "limit" => limit);
        let response = self.proxer.connect(&url, &body)?;
        let data: Response<Vec<News>> = serde_json::from_reader(response)?;
        check_error!(data.error, data.code.unwrap_or_default(), data.message);
        check_data!(data.data)
    }

    /// Löscht eine gegebene Notification
    ///
    /// # Arguments
    ///
    /// * `nid` - Die ID der zu löschenden Notification.
    /// Wenn weggelassen oder 0, so werden alle als gelesen markierten Benachrichtigungen gelöscht.
    pub fn delete_notification(&self, nid: Option<u64>) -> Result<()> {
        let url = url!("notifications", "delete");
        let body = param_build!("nid" => nid);
        let response = self.proxer.connect(&url, &body)?;
        let data: EmptyResponse = serde_json::from_reader(response)?;
        check_error!(data.error, 0, data.message);
        Ok(())
    }
}
