//! Inoffizielle Bibliothek zur proxer-api in rust.
//!
//! Die meisten Dokumentationen sind aus dem Proxer-API wiki.
//! Sollte etwas in der crate fehlen, oder anderweitig nicht stimmen, bitte auf Github melden.
//!
//! Aktuell unterstützte Rust Version >= 1.15
//!
//! # Auszug aus dem Proxer-API wiki
//!
//! Die Proxer API ist eine Programmierschnittstelle,
//! die dazu dient, Proxer in allen Aspekten erweiterbar zu machen.
//!
//! ##Nutzungsbedingungen
//! * Die Anwendung muss ausdrücklich als "inoffizielle Anwendung"
//! deklariert werden (z.B. beim Starten der Anwendung in Form eines Dialogs,
//! in der Anwendungsbeschreibung, im Impressum und/oder in den Credits).
//! * Die Anzahl der Anfragen an den Proxer-Server muss so gering gehalten werden wie nur möglich.
//! *Auf die Sicherheit der Anwendung muss stark geachtet werden.
//! Falls möglich, soll immer SSL zur Verschlüsselung der Kommunikation verwendet werden.
//! Gemeldete Sicherheitsprobleme müssen in kürzester Zeit behoben werden.
//! * In besonderen Fällen kann genesis zusätzliche Forderungen stellen, die dann ebenfalls eingehalten werden müssen.
//! * Die Nutzungsbedingungen könnten sich jederzeit ändern.
//! Nach einer Änderung müssen die Änderungen innerhalb von 30 Tagen umgesetzt werden.
//! Alle Änderungen werden in der Mailing-List erläutert.
//!
//! #Finanzielles
//! Nehmen wir an, es gibt eine "Kodi" Anwendung.
//! Wenn alle diese Anwendung verwenden würden, könnten wir unsere Server nicht mehr bezahlen.
//! Das wäre nachteilhaft für alle beteiligten.
//! Programmiere deine Anwendungen so, dass du es mit bestem Gewissen verwenden und verteilen kannst.
//! Prinzipiell werden wir dich nicht davon abhalten, z.B. mit Hilfe von Werbung Geld mit deiner Anwendung zu verdienen.
//! Unsere Bitte an dieser Stelle ist,
//! dass ihr unsere Spenden-Adresse irgendwo in die Anwendung einbaut (bei Android Anwendungen z.B. in das Menü),
//! oder ein Teil der Einnahmen spendet, damit wir weiterhin imstande sind unsere Server zu finanzieren.
//!
//! ## Verwaltung
//! * `Mailing-List`
//! Der Eintrag in der Mailing-List ist freiwillig.
//! Du kannst dich in unsere Mailing-List eintragen lassen um per Proxer-PN über eine Änderung an den Nutzungsbedingungen
//! oder der API benachrichtigt zu werden.
//! Schreibe hierfür eine kurze PN an genesis mit dem Inhalt: "Ich möchte in der API-Mailingliste eingetragen werden".
//! Wir laden dich dann in eine Proxer-Gruppe ein.
//! * `Open Source`
//! Open-Source Projekte haben die Möglichkeit, "mehr" als nur eine inoffizielle Anwendung zu bleiben.
//! Falls das Projekt z.B. auf github oder einer anderen Versionsverwaltung gehostet ist, besteht die Möglichkeit,
//! dass wir die Anwendung zu unseren offiziellen Anwendungen aufnehmen.
//! Offizielle Anwendungen werden wir beispielsweise auf unserer Facebook-Seite empfehlen und aktiv unterstützen.
//! * `Support`
//! Proxer stellt für den Support deiner Anwendung eine Forum-Kategorie zur Verfügung.
//! Hier kannst du für deine Anwendung einen Thread erstellen, welcher für Fragen, Vorschläge, Probleme und Kritik offen steht.
//! In den ersten Beitrag beschreibst du deine Anwendung, postest dein Changelog,
//! wie man deine Anwendung benutzt und ggf. weitere Links wie z.B. zum git-repository.
//! Werbung für deine private Webseite sind strengstens untersagt und ist gegen die Forumsregeln.
//! Für sonstige Diskussionen und initiierung neuer Projekte haben wir ebenfalls eine eigene Kategorie.
//!
//! ## Datenschutz
//! * `Passwörter`
//! Eingegebene Passwörter sollten stets entweder in geschützten Speicherbereichen
//! (z.B. SharedPreferences in Android) und/oder verschlüsselt gespeichert werden.
//! * `Statistiken`
//! Du darfst gerne Google Analytics verwenden, wir empfehlen jedoch die
//! datenschutzfreundliche(https://www.datenschutzbeauftragter-info.de/fachbeitraege/google-analytics-datenschutzkonform-einsetzen/)
//! Version zu verwenden.
//! * `Transparenz`
//! Deine Anwendung sollte möglichst transparent arbeiten und keine dubiosen Aktionen im Hintergrund durchführen.
//! Wir werden in bestimmten Abständen Anwendungen prüfen
//! und bei festgestelltem Missbrauch der Schnittstelle die Erlaubnis zur Nutzung der Anwendung entziehen.
//!
//! ## Technische Informationen
//! * `Ressourcen`
//! Wir bitten ausdrücklich, sparsam mit unseren Ressourcen umzugehen.
//! Das ist sehr wichtig, da unsere Server besonders an Wochenenden an ihren Grenzen sind.
//! Um dies zu erzielen, muss darauf geachtet werden, dass keine Anfrage unnötig gesendet wird.
//! Im Falle einer Anwendung, die darstellt ob eine neue Episode eines Animes erschienen ist,
//! würde es beispielsweise ausreichen, wenn die Abfrage höchstens alle 30 Minuten durchgeführt wird.
//! * `Firewall`
//! Proxer verwendet Cloudflare, um im Falle eines Ddos-Angriffes die Aufrechterhaltung unserer Dienste zu gewährleisten.
//! Aktuell besteht leider keine Möglichkeit, diese zu umgehen.
//! Fehlerhafte Anfragen sollten daher abgefangen werden, damit die Anwendung im Falle eines Angriffes nicht abstürzt.
//!
//! ## Dokumentation der Schnittstellen
//! Im Juni 2016 wurde eine Neuentwicklung der Schnittstellen durchgeführt.
//! Die alten Schnittstellen werden ab sofort nicht mehr unterstüzt, bleiben jedoch bis Januar 2017 weiterhin aktiv.
//! Die neue API ermöglicht eine Versionisierung der Schnittstellen, sodass eine nahtlose Weiterentwicklung stattfindet.
//!
//! * `Proxer-API Version 1:` Unterstützung ab Juni 2016. Aktuelle Version.
//!
//! #Weitere Fragen?
//! Du kannst dich jederzeit an genesis(http://proxer.me/wiki/Benutzer:Genesis) wenden.
//!
//! (Quelle:http://proxer.me/wiki/Proxer_API, vom 20.01.2017 18:00)

#![doc(html_logo_url = "", html_favicon_url = "")]

#[macro_use] extern crate hyper;
extern crate hyper_native_tls;
#[macro_use] extern crate serde_derive;
extern crate serde_json;
#[macro_use] extern crate log;

#[macro_use] mod macros;
pub mod error;
pub mod models;
pub mod anime;
pub mod info;
pub mod list;
pub mod manga;
pub mod media;
pub mod messenger;
pub mod notification;
pub mod ucp;
pub mod user;

use hyper::client::Client;
use hyper::header::{ Headers, ContentType, UserAgent };
use hyper::net::HttpsConnector;
use hyper_native_tls::NativeTlsClient;

use ::error::*;

static BASE_URL: &'static str = "https://proxer.me/api";
static API_VERSION: &'static str = "v1";
static USER_AGENT: &'static str = concat!("proxer-rs (https://github.com/souryo/proxer-rs, ",
    env!("CARGO_PKG_VERSION"), ")");

static NEWS_URL: &'static str = "http://proxer.me/notifications?format=json&s=news&p=1";

/// Ermöglicht den Zugriff auf die Proxer News. Hierbei werden pro Seite höchstens 15 News ausgegeben.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProxerNews
{
    /// 0 (erfolgreich) oder 1.
    pub error: u64,
    /// Eine Meldung im Falle eines Fehlers.
    pub message: Option<String>,
    /// Ein Array aus den Nachrichten-Objekten, im Falle einer erfolgreichen Ausgabe.
    pub notifications: Option<Vec<NewsNotification>>,
}

/// Beinhaltet eine einzelene News bzw. Notification (Benutzt die alte api Schnittstelle?).
///
/// # Eingabe
/// http://proxer.me/notifications?format=json&s=news&p=1
/// Der Parameter p gibt die aktuelle Seite an.
/// Hinweis: Aktuell wird der Abruf der Seitennummer ignoriert. Author: souryo(proxer-rs)
///
/// # Arguments
/// * `nid` - Die Nachrichten ID.
/// * `time` - Ein Unix Timestamp der angibt, wann die Nachricht veröffentlicht wurde.
/// * `mid` - Veraltet. Bitte stattdessen "thread" verwenden.
/// * `pid` - Veraltet.
/// * `description` - Eine Kurzbeschreibung zu der News.
/// * `image_id` - Die Bild-ID der News.
/// * `image_style` - Hier sind CSS Style-Angaben zum Bild gespeichert.
/// * `subject` - Die Überschrift der News.
/// * `hits` - Anzahl der News-Aufrufe.
/// * `thread` - Die Thread-ID. Ist äquivalent zu mid.
/// * `uid` - Die Benutzer-ID des Autors.
/// * `uname` - Der Benutzername des Autors.
/// * `posts` - Anzahl der Antworten/Kommentare auf die News.
/// * `catid` - Die Kategorie-ID der Kategorie, in der sich die News befindet.
/// * `catname` - Der Name der Kategorie, in der sich die News befindet.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NewsNotification
{
    /// News id
    pub nid: u64,
    /// Time als Unix-Timestamp
    pub time: i64,
    /// Die Beschreibung
    pub description: String,
    /// Die Bild id
    pub image_id: u64,
    /// Bildstyles
    pub image_style: String,
    pub subject: String,
    /// Aufrufe der Notification
    pub hits: u64,
    /// thread id
    pub thread: u64,
    /// Die Benutzer-ID des Autors.
    pub uid: u64,
    /// Der Benutzername des Autors.
    pub uname: String,
    /// Anzahl der Antworten/Kommentare auf die News.
    pub posts: u64,
    /// Die Kategorie-ID der Kategorie, in der sich die News befindet.
    pub catid: u64,
    /// Der Name der Kategorie, in der sich die News befindet.
    pub catname: String,
}

impl NewsNotification
{
    /// Der Link zum Bild ist folgendermaßen aufgebaut:
    /// http://cdn.proxer.me/news/{nid}_{image_id}.png
    /// Beachte, dass hier nur Thumbnails ausgegeben werden.
    /// Falls Zugriff auf die Originalbilder nötig ist,
    /// kann genesis(http://proxer.me/wiki/Benutzer:Genesis) hierzu angeschrieben werden.
    pub fn get_image(&self)
    -> String
    {
        format!("http://cdn.proxer.me/news/{}_{}.png", self.nid, self.image_id)
    }

    /// Der Link zur News ist folgendermaßen aufgebaut:
    /// http://proxer.me/forum/{catid}/{thread}
    /// Alternativ kann an den Link der Anker #top angegeben werden.
    pub fn get_news_link(&self)
    -> String
    {
        format!("http://proxer.me/forum/{}/{}", self.catid, self.thread)
    }
}

/// Klasse um die http/s Verbindungen aufzubauen und die API-Daten zu verwalten.
#[derive(Debug)]
pub struct Proxer
{
    client: hyper::Client,
    header: hyper::header::Headers,
}

// TODO use ssl/tls
impl Proxer
{
    /// Erstellt eine Proxer-Sitzung mit dem angegebenen API-Key.
    pub fn new(api_key: &str)
    -> Result<Self>
    {
        println!("proxer-rs ist eine inoffiziell Bibliothek!");

        let mut header = Headers::new();
        header.set(ContentType::form_url_encoded());
        header.set(UserAgent(USER_AGENT.to_owned()));

        header!{ (ProxerApiToken, "proxer-api-token") => [String] };
        header.set(ProxerApiToken(api_key.to_owned()));
        // TODO remove unwrap()
        let ssl = NativeTlsClient::new().unwrap();
        let connector = HttpsConnector::new(ssl);
        let client = Client::with_connector(connector);

        let proxer = Proxer
        {
            client: client,
            header: header,
        };
        Ok(proxer)
    }

    /// Funktion um, über die alte? News API, News abzurufen.
    /// Die ausgegebenen News stammen aus dem News-Feed der Startseite.
    pub fn get_news(&self)
    -> Result<Vec<NewsNotification>>
    {
        let url = NEWS_URL;
        let result = self.connect(&url, "")?;
        let data: ProxerNews = serde_json::from_reader(result)?;
        check_error!(data.error, 0, data.message.unwrap_or_default());
        check_data!(data.notifications)
    }

    // TODO try to remove clone() call, if possible
    fn connect(&self, url: &str, body: &str)
    -> Result<hyper::client::Response>
    {
        Ok(self.client.post(url)
            .headers(self.header.clone())
            .body(body)
            .send()?)
    }
}
