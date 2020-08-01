use actix_web::{get, web, Error, HttpResponse};
use diesel::r2d2::{self, ConnectionManager};
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
type DbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

use crate::mntconfig::Config;
use crate::bookings;
use crate::models::*;
use crate::schema::booking_docs::dsl::*;

use csv::{WriterBuilder,QuoteStyle};

#[derive(serde::Serialize,Default)]
struct DatevHeader {
    datev_format: String,
    version: String,
    kategorie: String,
    format_name: String,
    format_version: String,
    erzeugt_am: String,
    importiert: String,
    herkunft: String,
    exportiert_von: String,
    importiert_von: String,
    berater: String,
    mandant: String,
    wj_beginn: String,
    sachkonten_laenge: String,
    datum_von: String,
    datum_bis: String,
    bezeichnung: String,
    diktat_kuerzel: String,
    buchungstyp: String,
    rl_zweck: String,
    festschreibung: String,
    wkz: String,
    pad1: String,
    dkz: String,
    pad2: String,
    pad3: String,
    skr: String,
    blid: String,
    pad4: String,
    pad5: String,
    anwendungs_info: String
}

#[derive(serde::Serialize,Default)]
struct DatevBooking {
    umsatz: String,
    soll_haben: String,
    wkz: String,
    kurs: String,
    basis_umsatz: String,
    wkz_basis_umsatz: String,
    konto: String,
    gegenkonto: String,
    bu_schluessel: String,
    belegdatum: String,
    belegfeld_1: String,
    belegfeld_2: String,
    skonto: String,
    buchungstext: String,
    postensperre: String,
    pad1: String,
    pad2: String,
    pad3: String,
    pad4: String,
    beleglink: String,
    beleg_k1: String,
    beleg_v1: String,
    beleg_k2: String,
    beleg_v2: String,
    beleg_k3: String,
    beleg_v3: String,
    beleg_k4: String,
    beleg_v4: String,
    beleg_k5: String,
    beleg_v5: String,
    beleg_k6: String,
    beleg_v6: String,
    beleg_k7: String,
    beleg_v7: String,
    beleg_k8: String,
    beleg_v8: String,
    kost1: String,
    kost2: String,
    kost_menge: String,
    eu_ustid: String,
    eu_taxrate: String,
    pad5: String,
    pad6: String,
    pad7: String,
    pad8: String,
    pad9: String,
    pad10: String,
    zusatz_k1: String,
    zusatz_v1: String,
    zusatz_k2: String,
    zusatz_v2: String,
    zusatz_k3: String,
    zusatz_v3: String,
    zusatz_k4: String,
    zusatz_v4: String,
    zusatz_k5: String,
    zusatz_v5: String,
    zusatz_k6: String,
    zusatz_v6: String,
    zusatz_k7: String,
    zusatz_v7: String,
    zusatz_k8: String,
    zusatz_v8: String,
    zusatz_k9: String,
    zusatz_v9: String,
    zusatz_k10: String,
    zusatz_v10: String,
    zusatz_k11: String,
    zusatz_v11: String,
    zusatz_k12: String,
    zusatz_v12: String,
    zusatz_k13: String,
    zusatz_v13: String,
    zusatz_k14: String,
    zusatz_v14: String,
    zusatz_k15: String,
    zusatz_v15: String,
    zusatz_k16: String,
    zusatz_v16: String,
    zusatz_k17: String,
    zusatz_v17: String,
    zusatz_k18: String,
    zusatz_v18: String,
    zusatz_k19: String,
    zusatz_v19: String,
    zusatz_k20: String,
    zusatz_v20: String,
    stueck: String,
    gewicht: String,
    zahlweise: String,
    pad11: String,
    pad12: String,
    pad13: String,
    pad14: String,
    pad15: String,
    pad16: String,
    ust_schluessel: String,
    pad17: String,
    pad18: String,
    pad19: String,
    erloeskonto: String,
    herkunft_kz: String,
    buchungs_guid: String,
    kost_datum: String,
    sepa_mandat: String,
    skontosperre: String,
    pad20: String,
    pad21: String,
    pad22: String,
    pad23: String,
    pad24: String,
    pad25: String,
    pad26: String,
    festschreibung: String,
    leistungsdatum: String,
    pad27: String,
    pad28: String,
    generalumkehr: String,
    steuersatz: String,
    land: String
}

#[get("/bookings-datev")]
pub async fn get_bookings_datev_csv(
    pool: web::Data<DbPool>,
    q: web::Query<bookings::Query>,
    config: web::Data<Config>
) -> Result<HttpResponse, Error> {
    let conn = pool.get().expect("couldn't get db connection from pool");

    let bookings:Vec<Booking> = bookings::get_all_bookings(&conn, &q);
    
    let header1 = DatevHeader {
        datev_format: "EXTF".to_string(),
        version: "700".to_string(),
        kategorie: "21".to_string(),
        format_name: "Buchungsstapel".to_string(),
        format_version: "9".to_string(),
        herkunft: "RE".to_string(),
        exportiert_von: "mntbooks2".to_string(),
        berater: config.datev_advisor_id.clone(),
        mandant: config.datev_client_id.clone(),
        wj_beginn: "20200101".to_string(), // FIXME
        sachkonten_laenge: "4".to_string(),
        datum_von: "20200101".to_string(), // FIXME
        datum_bis: "20201231".to_string(), // FIXME
        festschreibung: "0".to_string(),
        wkz: "EUR".to_string(),
        ..Default::default()
    };
    
    let mut wtr_header = WriterBuilder::new()
        .delimiter(b';')
        .has_headers(false)
        .quote_style(QuoteStyle::Always)
        .from_writer(vec![]);
    
    wtr_header.serialize(header1).unwrap();
    
    let mut wtr = WriterBuilder::new()
        .delimiter(b';')
        .has_headers(false)
        .quote_style(QuoteStyle::Always)
        .from_writer(vec![]);
    
    wtr.write_record(["Umsatz (ohne Soll/Haben-Kz)","Soll/Haben-Kennzeichen","WKZ Umsatz","Kurs","Basis-Umsatz","WKZ Basis-Umsatz","Konto","Gegenkonto (ohne BU-Schlüssel)","BU-Schlüssel","Belegdatum","Belegfeld 1","Belegfeld 2","Skonto","Buchungstext","Postensperre","Diverse Adressnummer","Geschäftspartnerbank","Sachverhalt","Zinssperre","Beleglink","Beleginfo - Art 1","Beleginfo - Inhalt 1","Beleginfo - Art 2","Beleginfo - Inhalt 2","Beleginfo - Art 3","Beleginfo - Inhalt 3","Beleginfo - Art 4","Beleginfo - Inhalt 4","Beleginfo - Art 5","Beleginfo - Inhalt 5","Beleginfo - Art 6","Beleginfo - Inhalt 6","Beleginfo - Art 7","Beleginfo - Inhalt 7","Beleginfo - Art 8","Beleginfo - Inhalt 8","KOST1 - Kostenstelle","KOST2 - Kostenstelle","Kost-Menge","EU-Land u. UStID","EU-Steuersatz","Abw. Versteuerungsart","Sachverhalt L+L","Funktionsergänzung L+L","BU 49 Hauptfunktionstyp","BU 49 Hauptfunktionsnummer","BU 49 Funktionsergänzung","Zusatzinformation - Art 1","Zusatzinformation- Inhalt 1","Zusatzinformation - Art 2","Zusatzinformation- Inhalt 2","Zusatzinformation - Art 3","Zusatzinformation- Inhalt 3","Zusatzinformation - Art 4","Zusatzinformation- Inhalt 4","Zusatzinformation - Art 5","Zusatzinformation- Inhalt 5","Zusatzinformation - Art 6","Zusatzinformation- Inhalt 6","Zusatzinformation - Art 7","Zusatzinformation- Inhalt 7","Zusatzinformation - Art 8","Zusatzinformation- Inhalt 8","Zusatzinformation - Art 9","Zusatzinformation- Inhalt 9","Zusatzinformation - Art 10","Zusatzinformation- Inhalt 10","Zusatzinformation - Art 11","Zusatzinformation- Inhalt 11","Zusatzinformation - Art 12","Zusatzinformation- Inhalt 12","Zusatzinformation - Art 13","Zusatzinformation- Inhalt 13","Zusatzinformation - Art 14","Zusatzinformation- Inhalt 14","Zusatzinformation - Art 15","Zusatzinformation- Inhalt 15","Zusatzinformation - Art 16","Zusatzinformation- Inhalt 16","Zusatzinformation - Art 17","Zusatzinformation- Inhalt 17","Zusatzinformation - Art 18","Zusatzinformation- Inhalt 18","Zusatzinformation - Art 19","Zusatzinformation- Inhalt 19","Zusatzinformation - Art 20","Zusatzinformation- Inhalt 20","Stück","Gewicht","Zahlweise","Forderungsart","Veranlagungsjahr","Zugeordnete Fälligkeit","Skontotyp","Auftragsnummer","Buchungstyp (Anzahlungen)","USt-Schlüssel (Anzahlungen)","EU-Land (Anzahlungen)","Sachverhalt L+L (Anzahlungen)","EU-Steuersatz (Anzahlungen)","Erlöskonto (Anzahlungen)","Herkunft-Kz","Buchungs GUID","KOST-Datum","SEPA-Mandatsreferenz","Skontosperre","Gesellschaftername","Beteiligtennummer","Identifikationsnummer","Zeichnernummer","Postensperre bis","Bezeichnung SoBil-Sachverhalt","Kennzeichen SoBil-Buchung","Festschreibung","Leistungsdatum","Datum Zuord. Steuerperiode","Fälligkeit","Generalumkehr (GU)","Steuersatz","Land"].iter()).unwrap();
    
    for booking in bookings {
        let mut soll_haben = "S".to_string();
        let mut amt = booking.amount_cents/100;
        let mut cents = booking.amount_cents%100;

        let skip = if let Some(url) = booking.receipt_url {
            url.is_empty()
        } else {
            true
        };

        let docs:Vec<BookingDoc> = booking_docs.limit(1).filter(booking_id.eq(&booking.id))
            .load::<BookingDoc>(&conn).unwrap();

        if !skip {
            if let Some(booking_doc) = docs.first() {
                // find account that is not the asset side
                let acc_str = if booking.credit_account.starts_with("assets:") {
                    booking.debit_account.clone()
                } else {
                    booking.credit_account.clone()
                };

                if booking.debit_account.starts_with("sales:") {
                    soll_haben = "H".to_string();
                }

                // FIXME this should actually never happen. data error?
                if amt<0 || cents<0 {
                    amt = -amt;
                    cents = -cents;
                    soll_haben = "H".to_string();
                    // TODO: generalumkehr?
                }

                let mut account1 = "9999".to_string();
                let mut account2 = "99999".to_string();

                for (match_str, target_acc) in config.datev_account1_map.iter() {
                    if acc_str.contains(match_str) {
                        account1 = target_acc.clone();
                    }
                }

                // TODO include documentimage's tags in this search
                for (match_str, target_acc) in config.datev_account2_map.iter() {
                    if acc_str.contains(match_str) {
                        account2 = target_acc.clone();
                    }
                }

                let mon = &booking.booking_date[5..7];
                let day = &booking.booking_date[8..10];

                let belegfeld = match &booking_doc.doc_id {
                    Some(d) => {
                        let parts:Vec<&str> = d.split(',').collect();
                        parts.first().unwrap().to_string()
                    }
                    _ => "".to_string()
                };

                let d = DatevBooking {
                    umsatz: format!("{},{:02}",amt,cents),
                    wkz: booking.currency,

                    konto: account1,
                    gegenkonto: account2,
                    soll_haben: soll_haben,

                    belegfeld_1: belegfeld,
                    belegdatum: format!("{:02}{:02}",day,mon),
                    buchungstext: acc_str.clone(),

                    ust_schluessel: "0".to_string(),
                    erloeskonto: "0".to_string(),
                    herkunft_kz: "RE".to_string(),
                    skontosperre: "0".to_string(),
                    festschreibung: "0".to_string(),
                    generalumkehr: "0".to_string(),
                    ..Default::default()
                };

                wtr.serialize(d).unwrap();
            }
        }
    }
    let data = String::from_utf8(wtr_header.into_inner().unwrap()).unwrap() + &(String::from_utf8(wtr.into_inner().unwrap()).unwrap());
    Ok(HttpResponse::Ok().content_type("text/plain").body(data))
}
