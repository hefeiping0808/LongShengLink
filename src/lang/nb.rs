lazy_static::lazy_static! {
pub static ref T: std::collections::HashMap<&'static str, &'static str> =
    [
        ("Status", "Status"),
        ("Your Desktop", "Ditt skrivebord"),
        ("desk_tip", "Du kan få adgang til ditt skrivebord med denne ID og passord."),
        ("Password", "Passord"),
        ("Ready", "Klar"),
        ("Established", "Etablert"),
        ("connecting_status", "Opretter tilkobling til LongShengLink-nettverket..."),
        ("Enable service", "Aktiver tjenesten"),
        ("Start service", "Start tjenesten"),
        ("Service is running", "Tjenesten kjører"),
        ("Service is not running", " tilknyttede tjenesten kjører ikke"),
        ("not_ready_status", "Ikke klar. Vennligst sjekk din tilkobling"),
        ("Control Remote Desktop", "Kontroller fjernskrivebord"),
        ("Transfer file", "Overfør fil"),
        ("Connect", "Koble til"),
        ("Recent sessions", "Siste sesjoner"),
        ("Address book", "Adressebok"),
        ("Confirmation", "Bekreftelse"),
        ("TCP tunneling", "TCP tunnelering"),
        ("Remove", "Fjern"),
        ("Refresh random password", "Oppdater tilfeldig passord"),
        ("Set your own password", "Sett ditt eget passord"),
        ("Enable keyboard/mouse", "Aktiver tastatur/mus"),
        ("Enable clipboard", "Aktiver utklipstavle"),
        ("Enable file transfer", "Aktiver filoverførsel"),
        ("Enable TCP tunneling", "Aktiver TCP-tunnelering"),
        ("IP Whitelisting", "IP Hvitelisting"),
        ("ID/Relay Server", "ID/Tilkoblingsserver"),
        ("Import server config", "Importer serverkonfigurasjon"),
        ("Export Server Config", "Eksporter serverkonfigurasjon"),
        ("Import server configuration successfully", "Import av serverkonfigurasjonen var vellykket"),
        ("Export server configuration successfully", "Eksport av serverkonfigurasjonen var vellykket"),
        ("Invalid server configuration", "Ugyldig serverkonfigurasjon"),
        ("Clipboard is empty", "Utklipstavlen er tom"),
        ("Stop service", "Stopp tilkoblingsserveren"),
        ("Change ID", "Endre ID"),
        ("Your new ID", "Din nye ID"),
        ("length %min% to %max%", ""),
        ("starts with a letter", ""),
        ("allowed characters", ""),
        ("id_change_tip", "Kun tegnene a-z, A-Z, 0-9 og _ (understrek) er tillat. Den første bokstaven skal være a-z, A-Z. Lengde mellom 6 og 16."),
        ("Website", "Hjemmeside"),
        ("About", "Om"),
        ("Slogan_tip", ""),
        ("Privacy Statement", ""),
        ("Mute", "Deaktiver mikrofonen"),
        ("Build Date", ""),
        ("Version", "Versjon"),
        ("Home", "Hjem"),
        ("Audio Input", "Lydinput"),
        ("Enhancements", "Forbedringer"),
        ("Hardware Codec", "Hardware-codec"),
        ("Adaptive bitrate", "Adaptiv bitrate"),
        ("ID Server", "ID Server"),
        ("Relay Server", "Relay Server"),
        ("API Server", "API Server"),
        ("invalid_http", "Skal starte med http:// eller https://"),
        ("Invalid IP", "Ugyldig IP-adresse"),
        ("Invalid format", "Ugyldig format"),
        ("server_not_support", "Enda ikke støttet av serveren"),
        ("Not available", "Ikke tilgengelig"),
        ("Too frequent", "For hyppig"),
        ("Cancel", "Avbryt"),
        ("Skip", "Hopp over"),
        ("Close", "Lukk"),
        ("Retry", "Prøv igjen"),
        ("OK", "OK"),
        ("Password Required", "passord påkrevd"),
        ("Please enter your password", "Tast inn ditt passord"),
        ("Remember password", "Husk passord"),
        ("Wrong Password", "Feil passord"),
        ("Do you want to enter again?", "Ønsker du å forsøke igen?"),
        ("Connection Error", "Tilkoblingsfeil"),
        ("Error", "Feil"),
        ("Reset by the peer", "Nulstilt av motparten"),
        ("Connecting...", "Opretter tilkobling..."),
        ("Connection in progress. Please wait.", "Tilkobler. Vennligst vent."),
        ("Please try 1 minute later", "Prøv igen om et minutt"),
        ("Login Error", "Login feil"),
        ("Successful", "Vellykket"),
        ("Connected, waiting for image...", "Tilkoblet, venter på bilde..."),
        ("Name", "Navn"),
        ("Type", "Type"),
        ("Modified", "Endret"),
        ("Size", "Størrelse"),
        ("Show Hidden Files", "Vis skjulte filer"),
        ("Receive", "Motta"),
        ("Send", "Send"),
        ("Refresh File", "Oppdater fil"),
        ("Local", "Lokalt"),
        ("Remote", "Remote"),
        ("Remote Computer", "fjerntilkoblet maskin"),
        ("Local Computer", "Lokal maskin"),
        ("Confirm Delete", "Bekreft sletting"),
        ("Delete", "Slett"),
        ("Properties", "Egenskaper"),
        ("Multi Select", "Flere valg"),
        ("Select All", "Velg alle"),
        ("Unselect All", "Fravelg alle"),
        ("Empty Directory", "Tomt bibliotek"),
        ("Not an empty directory", "Ikke et tomt bibliotek"),
        ("Are you sure you want to delete this file?", "Er du sikker på, at du vil slette denne filen?"),
        ("Are you sure you want to delete this empty directory?", "Er du sikker på, at du vil slette dette tomme biblioteket?"),
        ("Are you sure you want to delete the file of this directory?", "Er du sikker på, at du vil slette filen til dette biblioteket?"),
        ("Do this for all conflicts", "Gjør dette for alle konflikter"),
        ("This is irreversible!", "Dette kan ikke reverseres!"),
        ("Deleting", "Sletter"),
        ("files", "Filer"),
        ("Waiting", "Venter"),
        ("Finished", "Ferdig"),
        ("Speed", "Hastighet"),
        ("Custom Image Quality", "Brukerdefinert bildekvalitet"),
        ("Privacy mode", "Privatlivsmodus"),
        ("Block user input", "Blokker brukerinput"),
        ("Unblock user input", "Fjern blokkering av brukerinput"),
        ("Adjust Window", "Juster vinduet"),
        ("Original", "Original"),
        ("Shrink", "Krymp"),
        ("Stretch", "Strekk ut"),
        ("Scrollbar", "Rullebar"),
        ("ScrollAuto", "Auto-rull"),
        ("Good image quality", "God bildekvalitet"),
        ("Balanced", "Balansert"),
        ("Optimize reaction time", "Optimert responstid"),
        ("Custom", "Tilpasset"),
        ("Show remote cursor", "Vis fjernstyrt musepeker"),
        ("Show quality monitor", "Vis bildekvalitet"),
        ("Disable clipboard", "Deaktiver utklipstavle"),
        ("Lock after session end", "Lås etter avsluttet fjernstyring"),
        ("Insert Ctrl + Alt + Del", "Sett inn Ctrl + Alt + Del"),
        ("Insert Lock", "Sett inn lås"),
        ("Refresh", "Oppdater"),
        ("ID does not exist", "ID finnes ikke"),
        ("Failed to connect to rendezvous server", "tilkobling til serveren mislykktes"),
        ("Please try later", "Prøv igjen senere"),
        ("Remote desktop is offline", "Fjernskrivebord er offline"),
        ("Key mismatch", "Nøkkel mismatch"),
        ("Timeout", "Timeout"),
        ("Failed to connect to relay server", "tilkobling til rele-serveren mislykktes"),
        ("Failed to connect via rendezvous server", "tilkobling via Rendezvous-server mislykktes"),
        ("Failed to connect via relay server", "tilkobling via rele-serveren mislykktes"),
        ("Failed to make direct connection to remote desktop", "Direkte tilkobling til fjernskrivebord kunne ikke etableres"),
        ("Set Password", "Sett passord"),
        ("OS Password", "Operativsystempassord"),
        ("install_tip", "På grunn av UAC kan LongShengLink ikke fungere korrekt i enkelte tillfeller på fjernskrivebordet. For å unngå UAC klikker du på knappen nedenfor for å installere LongShengLink på systemet"),
        ("Click to upgrade", "Klikk for å oppgradere"),
        ("Click to download", "Klikk for å laste ned"),
        ("Click to update", "Klikk for å oppdatere"),
        ("Configure", "Konfigurer"),
        ("config_acc", "For å kontrollere ditt skrivebord med fjernstyring må du gi LongShengLink \"Access \" Rettigheter."),
        ("config_screen", "For å kunne få adgang til ditt skrivebord med fjernstyring, må du gi LongShengLink \"skjerstøtte \" tillatelser."),
        ("Installing ...", "Installerer ..."),
        ("Install", "installer"),
        ("Installation", "Installasjon"),
        ("Installation Path", "Installasjonssti"),
        ("Create start menu shortcuts", "Oppret start meny snarvei"),
        ("Create desktop icon", "Oppret skrivebords-snarvei"),
        ("agreement_tip", "Hvis du starter installasjonen, må du akseptere lisensavtalen"),
        ("Accept and Install", "Aksepter og installer"),
        ("End-user license agreement", "Lisensavtale for sluttbrukere"),
        ("Generating ...", "Genererer kode ..."),
        ("Your installation is lower version.", "Din installasjon er en eldre versjon."),
        ("not_close_tcp_tip", "Ikke lukk dette vinduet, mens du bruker tunnelen."),
        ("Listening ...", "Lytter ..."),
        ("Remote Host", "Fjern-Host"),
        ("Remote Port", "Fjern-Port"),
        ("Action", "Handling"),
        ("Add", "Tilføy"),
        ("Local Port", "Lokal Port"),
        ("Local Address", "Lokal adresse"),
        ("Change Local Port", "Skift lokal port"),
        ("setup_server_tip", "For en hurtigere tilkobling må du bruke din egen tilkoblingsserver"),
        ("Too short, at least 6 characters.", "For kort, bruk minst 6 tegn."),
        ("The confirmation is not identical.", "bekreftelsen er ikke identisk."),
        ("Permissions", "Tillatelser"),
        ("Accept", "Aksepter"),
        ("Dismiss", "Avvis"),
        ("Disconnect", "Koble fra"),
        ("Enable file copy and paste", "Tillat kopiering og innliming av filer"),
        ("Connected", "Tilkoblet"),
        ("Direct and encrypted connection", "Direkte og kryptert tilkobling"),
        ("Relayed and encrypted connection", "Viderekoblet og kryptert tilkobling"),
        ("Direct and unencrypted connection", "Direkte og ukryptert tilkobling"),
        ("Relayed and unencrypted connection", "Viderekoblet og ukryptert tilkobling"),
        ("Enter Remote ID", "Tast inn Remote-ID"),
        ("Enter your password", "Skriv ditt passord"),
        ("Logging in...", "Logger inn..."),
        ("Enable RDP session sharing", "Aktiver RDP sesjonsgodkjennelse"),
        ("Auto Login", "Automatisk login (kun gyldig hvis du har konfigurert \"Lås etter avslutting av sesjonen\")"),
        ("Enable direct IP access", "Aktiver direkte IP-adgang"),
        ("Rename", "Gi nytt navn"),
        ("Space", "Plass"),
        ("Create desktop shortcut", "Opprett skrivebords-snarvei"),
        ("Change Path", "Skift sti"),
        ("Create Folder", "Opprett mappe"),
        ("Please enter the folder name", "Tast inn mappens navn"),
        ("Fix it", "Kjør reparasjon"),
        ("Warning", "Advarsel"),
        ("Login screen using Wayland is not supported", "Login skjerm med Wayland støttes ikke"),
        ("Reboot required", "Omstart kreves"),
        ("Unsupported display server", "Ikke-understøttet displayserver"),
        ("x11 expected", "X11 Forventet"),
        ("Port", "Port"),
        ("Settings", "Innstillinger"),
        ("Username", " Brukernavn"),
        ("Invalid port", "Ugyldig port"),
        ("Closed manually by the peer", "Manuelt lukket av peer"),
        ("Enable remote configuration modification", "Tillat fjernkonfigurering"),
        ("Run without install", "Kjør uten installasjon"),
        ("Connect via relay", "Koble til via viderekobling"),
        ("Always connect via relay", "tilkobling via viderekoblings-server"),
        ("whitelist_tip", "Kun IP'er på hvitelisten kan få adgang til meg"),
        ("Login", "Logg inn"),
        ("Verify", "Verifiser"),
        ("Remember me", "Husk meg"),
        ("Trust this device", "Husk denne enheten"),
        ("Verification code", "Verifikasjonskode"),
        ("verification_tip", ""),
        ("Logout", "Logger av"),
        ("Tags", "Tagger"),
        ("Search ID", "Søk etter ID"),
        ("whitelist_sep", "Adskilt etter komma, semikolon, mellemrom eller linjeskift"),
        ("Add ID", "Legg til ID"),
        ("Add Tag", "Legg til tagg"),
        ("Unselect all tags", "Fravelg alle passord"),
        ("Network error", "Nettverksfeil"),
        ("Username missed", "Glemt brukernavn"),
        ("Password missed", "Glemt passord"),
        ("Wrong credentials", "Feil brukernavn og/eller passord"),
        ("The verification code is incorrect or has expired", ""),
        ("Edit Tag", "Rediger tagg"),
        ("Forget Password", "Glem passord"),
        ("Favorites", "Favoritter"),
        ("Add to Favorites", "Legg til favoritter"),
        ("Remove from Favorites", "Fjern favoritter"),
        ("Empty", "Tom"),
        ("Invalid folder name", "Ugyldig mappenavn"),
        ("Socks5 Proxy", "Socks5 Proxy"),
        ("Socks5/Http(s) Proxy", "Socks5/Http(s) Proxy"),
        ("Discovered", "Oppdaget"),
        ("install_daemon_tip", "For å starte når PC'en har startet opp, må du installere systemtjenesten"),
        ("Remote ID", "Fjern-ID"),
        ("Paste", "Sett inn"),
        ("Paste here?", "Sett inn her?"),
        ("Are you sure to close the connection?", "Er du sikker på at du vil lukke tilkoblingn?"),
        ("Download new version", "Last ned ny versjon"),
        ("Touch mode", "Touch-modus"),
        ("Mouse mode", "Muse-modus"),
        ("One-Finger Tap", "En-finger-trykk"),
        ("Left Mouse", "Venstre mus"),
        ("One-Long Tap", "Trykk og hold med en finger"),
        ("Two-Finger Tap", "Trykk med to fingre"),
        ("Right Mouse", "Høyre mus"),
        ("One-Finger Move", "En-finger bevegelse"),
        ("Double Tap & Move", "Dobbeltklikk og flytt"),
        ("Mouse Drag", "Dra med musen"),
        ("Three-Finger vertically", "Tre fingre lodrett"),
        ("Mouse Wheel", "Musehjul"),
        ("Two-Finger Move", "To-finger bevegelse"),
        ("Canvas Move", "Flytt lerret"),
        ("Pinch to Zoom", "Knip for å zoome inn"),
        ("Canvas Zoom", "Lerret zoom"),
        ("Reset canvas", "Nullstill lerret"),
        ("No permission of file transfer", "Ingen tillatelse til å overføre filen"),
        ("Note", "Notat"),
        ("Connection", "Tilkobling"),
        ("Share Screen", "Del skjermen"),
        ("Chat", "Chat"),
        ("Total", "Total"),
        ("items", "Objekter"),
        ("Selected", "Valgte"),
        ("Screen Capture", "Skjermopptak"),
        ("Input Control", "Input kontroll"),
        ("Audio Capture", "Lydopptak"),
        ("File Connection", "Filtilkobling"),
        ("Screen Connection", "Skjermtilkobing"),
        ("Do you accept?", "Akepterer du?"),
        ("Open System Setting", "Åpne systeminnstillinger"),
        ("How to get Android input permission?", "Hvordan får jeg en Android-input tillatelse?"),
        ("android_input_permission_tip1", "For at en ekstern enhet kan kontrollere din Android-enhet via mus eller berøring, må du gi LongShengLink mulighet til å bruke tjenesten \"tilgjengelighet \"."),
        ("android_input_permission_tip2", "Gå til den neste systeminnstillingssiden, søk og tast inn [installerte tjenester], aktiver [LongShengLink Input] tjenesten."),
        ("android_new_connection_tip", "En ny forespørsel ble mottatt, som ønsker å kontrollere din nåværende enhet."),
        ("android_service_will_start_tip", "Ved å aktivere skjermopptak startes tjenesten automatisk, så andre enheter kan forespørre en tilkobling fra denne enheten."),
        ("android_stop_service_tip", "Ved å lukke tjenesten lukkes alle tilkoblinger automatisk."),
        ("android_version_audio_tip", "Den aktuelle Android-versjonen støtter ikke lydopptak. Android 10 eller nyere kreves."),
        ("android_start_service_tip", ""),
        ("android_permission_may_not_change_tip", ""),
        ("Account", "Konto"),
        ("Overwrite", "Overskriv"),
        ("This file exists, skip or overwrite this file?", "Denne filen finnes allerede, vil du hoppe over eller overskrive denne filen?"),
        ("Quit", "Avslutt"),
        ("Help", "Hjelp"),
        ("Failed", "Mislykket"),
        ("Succeeded", "Vellykket"),
        ("Someone turns on privacy mode, exit", "Noen aktiverte privatlivsmodus, avslutt"),
        ("Unsupported", "Ikke støttet"),
        ("Peer denied", "Motpart nektet"),
        ("Please install plugins", "Installer plugins"),
        ("Peer exit", "Motpart-Avslutt"),
        ("Failed to turn off", "Klarte ikke å skru av"),
        ("Turned off", "Avslått"),
        ("Language", "Språk"),
        ("Keep LongShengLink background service", "Behold LongShengLink baggrundstjeneste"),
        ("Ignore Battery Optimizations", "Ignorer batteri optimalisering"),
        ("android_open_battery_optimizations_tip", ""),
        ("Start on boot", "Start under oppstart"),
        ("Start the screen sharing service on boot, requires special permissions", "Start skjermdelingstjenesten under oppstart, krever spesielle tillatelser"),
        ("Connection not allowed", "tilkobling ikke tillat"),
        ("Legacy mode", "Tilbakekompatibilitetstilstand"),
        ("Map mode", "Kartmodus"),
        ("Translate mode", "Oversettelsesmodus"),
        ("Use permanent password", "Bruk permanent passord"),
        ("Use both passwords", "Bruk begge passord"),
        ("Set permanent password", "Sett permanent passord"),
        ("Enable remote restart", "Aktiver fjerngomstart"),
        ("Restart remote device", "Restart fjernenhed"),
        ("Are you sure you want to restart", "Er du sikker på at du vil restarte"),
        ("Restarting remote device", "Restarter fjernenhet"),
        ("remote_restarting_tip", "Enheten starter på nytt - Lukker denne beskjeden og kobler til igjen om et øyeblikk"),
        ("Copied", "Kopiert"),
        ("Exit Fullscreen", "Avslutt fullskjerm"),
        ("Fullscreen", "Fullskjerm"),
        ("Mobile Actions", "Mobile handlinger"),
        ("Select Monitor", "velg skjerm"),
        ("Control Actions", "Kontrollhandlinger"),
        ("Display Settings", "Skjerminnstillinger"),
        ("Ratio", "Forhold"),
        ("Image Quality", "Bildekvalitet"),
        ("Scroll Style", "Rullestil"),
        ("Show Toolbar", "Vis Verktøylinje"),
        ("Hide Toolbar", "Skjul Verktøylinje"),
        ("Direct Connection", "Direkte tilkobling"),
        ("Relay Connection", "Viderekoblet tilkobling"),
        ("Secure Connection", "Sikker tilkobling"),
        ("Insecure Connection", "Usikker tilkobling"),
        ("Scale original", "Original skalering"),
        ("Scale adaptive", "Adaptiv skalering"),
        ("General", "Generelt"),
        ("Security", "Sikkerhet"),
        ("Theme", "Tema"),
        ("Dark Theme", "Mørkt Tema"),
        ("Light Theme", "Lyst Tema"),
        ("Dark", "Mørk"),
        ("Light", "Lys"),
        ("Follow System", "Følg System"),
        ("Enable hardware codec", "Aktiver hardware-codec"),
        ("Unlock Security Settings", "Lås opp Sikkerhetsinnstillinger"),
        ("Enable audio", "Aktiver Lyd"),
        ("Unlock Network Settings", "Lås opp Nettverksinnstillinger"),
        ("Server", "Server"),
        ("Direct IP Access", "Direkte IP Adgang"),
        ("Proxy", "Proxy"),
        ("Apply", "Bruk"),
        ("Disconnect all devices?", "Koble fra alle enheter?"),
        ("Clear", "Nullstill"),
        ("Audio Input Device", "Lydinngangsenhet"),
        ("Use IP Whitelisting", "Bruk IP hvitelisting"),
        ("Network", "Nettverk"),
        ("Pin Toolbar", "Fest Hurtiglinje"),
        ("Unpin Toolbar", "Avfest Hurtiglinje"),
        ("Recording", "Opptak"),
        ("Directory", "Mappe"),
        ("Automatically record incoming sessions", "Ta opp innkommende sesjoner automatisk"),
        ("Automatically record outgoing sessions", ""),
        ("Change", "Rediger"),
        ("Start session recording", "Start sesjonsopptak"),
        ("Stop session recording", "Stopp sesjonsopptak"),
        ("Enable recording session", "Aktiver opptakssesjon"),
        ("Enable LAN discovery", "Aktiver LAN Discovery"),
        ("Deny LAN discovery", "Avvis LAN Discovery"),
        ("Write a message", "Skriv en beskjed"),
        ("Prompt", "Prompt"),
        ("Please wait for confirmation of UAC...", "Vennligst vent på UAC-bekreftelse..."),
        ("elevated_foreground_window_tip", ""),
        ("Disconnected", "Frakoblet"),
        ("Other", "Andre"),
        ("Confirm before closing multiple tabs", "Bekreft før du lukker flere faner"),
        ("Keyboard Settings", "Tastaturinnstillinger"),
        ("Full Access", "Full tilgang"),
        ("Screen Share", "Skjermdeling"),
        ("Wayland requires Ubuntu 21.04 or higher version.", "Wayland krever Ubuntu version 21.04 eller nyere."),
        ("Wayland requires higher version of linux distro. Please try X11 desktop or change your OS.", "Wayland krever en nyere versjon av Linux. Prøv X11 desktop eller skift OS."),
        ("JumpLink", "JumpLink"),
        ("Please Select the screen to be shared(Operate on the peer side).", "vennligst velg den skjermen, som skal deles (fjernstyres)."),
        ("Show LongShengLink", "Vis LongShengLink"),
        ("This PC", "Denne PC"),
        ("or", "eller"),
        ("Continue with", "Fortsett med"),
        ("Elevate", "Elever"),
        ("Zoom cursor", "Zoom markør"),
        ("Accept sessions via password", "Aksepter sesjoner via passord"),
        ("Accept sessions via click", "Aksepter sesjoner via klikk"),
        ("Accept sessions via both", "Aksepter sesjoner via begge"),
        ("Please wait for the remote side to accept your session request...", "Vennligst vent på at fjernklienten aksepterer din sesjonsforespørsel..."),
        ("One-time Password", "Engangskode"),
        ("Use one-time password", "Bruk engangskode"),
        ("One-time password length", "Engangskode lengde"),
        ("Request access to your device", "Etterspør adgang til din enhet"),
        ("Hide connection management window", "Skjul tilkoblingshåndteringsvinduet"),
        ("hide_cm_tip", ""),
        ("wayland_experiment_tip", ""),
        ("Right click to select tabs", "Høyreklikk for å velge faner"),
        ("Skipped", "Hoppet over"),
        ("Add to address book", "Legg til adresseboken"),
        ("Group", "Gruppe"),
        ("Search", "Søk"),
        ("Closed manually by web console", "Lukket ned manuelt av webkonsollet"),
        ("Local keyboard type", "Lokal tastatur type"),
        ("Select local keyboard type", "velg lokal tastatur type"),
        ("software_render_tip", ""),
        ("Always use software rendering", "Bruk alltid programvare rendering"),
        ("config_input", ""),
        ("config_microphone", ""),
        ("request_elevation_tip", ""),
        ("Wait", "Vent"),
        ("Elevation Error", "Eleveringsfeil"),
        ("Ask the remote user for authentication", "Spør fjernbrukeren om godkjennelse"),
        ("Choose this if the remote account is administrator", "velg dette hvis fjernbrukeren er en administrator"),
        ("Transmit the username and password of administrator", "Send brukernavnet og passordet for administrator"),
        ("still_click_uac_tip", ""),
        ("Request Elevation", "Etterspørr elevering"),
        ("wait_accept_uac_tip", ""),
        ("Elevate successfully", "Elevering vellykket"),
        ("uppercase", "store bokstaver"),
        ("lowercase", "små bokstaver"),
        ("digit", "siffer"),
        ("special character", "spesialtegn"),
        ("length>=8", "lengde>=8"),
        ("Weak", "Svak"),
        ("Medium", "Medium"),
        ("Strong", "Sterk"),
        ("Switch Sides", "Skift sider"),
        ("Please confirm if you want to share your desktop?", "Bekreft at du ønsker å dele skrivebordet ditt?"),
        ("Display", "Visning"),
        ("Default View Style", "Standard visningsstil"),
        ("Default Scroll Style", "Standard rulle stil"),
        ("Default Image Quality", "Standard bildekvalitet"),
        ("Default Codec", "Standard codec"),
        ("Bitrate", "Bitrate"),
        ("FPS", "FPS"),
        ("Auto", "Auto"),
        ("Other Default Options", "Andre standardinnstillinger"),
        ("Voice call", "Stemmeoppkald"),
        ("Text chat", "Tekstchat"),
        ("Stop voice call", "Stopp stemmeoppkald"),
        ("relay_hint_tip", ""),
        ("Reconnect", "Gjenopprett"),
        ("Codec", "Codec"),
        ("Resolution", "Oppløsning"),
        ("No transfers in progress", "Ingen aktive overførsler"),
        ("Set one-time password length", "Sett engangspassord lengde"),
        ("RDP Settings", "RDP innstillinger"),
        ("Sort by", "Sorter etter"),
        ("New Connection", "Ny tilkobling"),
        ("Restore", "gjenopprett"),
        ("Minimize", "Minimer"),
        ("Maximize", "Maksimer"),
        ("Your Device", "Din enhet"),
        ("empty_recent_tip", ""),
        ("empty_favorite_tip", ""),
        ("empty_lan_tip", ""),
        ("empty_address_book_tip", ""),
        ("eg: admin", "f.eks.: admin"),
        ("Empty Username", "Tøm brukernavn"),
        ("Empty Password", "Tøm passord"),
        ("Me", "Meg"),
        ("identical_file_tip", ""),
        ("show_monitors_tip", ""),
        ("View Mode", ""),
        ("login_linux_tip", ""),
        ("verify_rustdesk_password_tip", ""),
        ("remember_account_tip", ""),
        ("os_account_desk_tip", ""),
        ("OS Account", ""),
        ("another_user_login_title_tip", ""),
        ("another_user_login_text_tip", ""),
        ("xorg_not_found_title_tip", ""),
        ("xorg_not_found_text_tip", ""),
        ("no_desktop_title_tip", ""),
        ("no_desktop_text_tip", ""),
        ("No need to elevate", ""),
        ("System Sound", ""),
        ("Default", ""),
        ("New RDP", ""),
        ("Fingerprint", ""),
        ("Copy Fingerprint", ""),
        ("no fingerprints", ""),
        ("Select a peer", ""),
        ("Select peers", ""),
        ("Plugins", ""),
        ("Uninstall", ""),
        ("Update", ""),
        ("Enable", ""),
        ("Disable", ""),
        ("Options", ""),
        ("resolution_original_tip", ""),
        ("resolution_fit_local_tip", ""),
        ("resolution_custom_tip", ""),
        ("Collapse toolbar", ""),
        ("Accept and Elevate", ""),
        ("accept_and_elevate_btn_tooltip", ""),
        ("clipboard_wait_response_timeout_tip", ""),
        ("Incoming connection", ""),
        ("Outgoing connection", ""),
        ("Exit", ""),
        ("Open", ""),
        ("logout_tip", ""),
        ("Service", ""),
        ("Start", ""),
        ("Stop", ""),
        ("exceed_max_devices", ""),
        ("Sync with recent sessions", ""),
        ("Sort tags", ""),
        ("Open connection in new tab", ""),
        ("Move tab to new window", ""),
        ("Can not be empty", ""),
        ("Already exists", ""),
        ("Change Password", ""),
        ("Refresh Password", ""),
        ("ID", ""),
        ("Grid View", ""),
        ("List View", ""),
        ("Select", ""),
        ("Toggle Tags", ""),
        ("pull_ab_failed_tip", ""),
        ("push_ab_failed_tip", ""),
        ("synced_peer_readded_tip", ""),
        ("Change Color", ""),
        ("Primary Color", ""),
        ("HSV Color", ""),
        ("Installation Successful!", ""),
        ("Installation failed!", ""),
        ("Reverse mouse wheel", ""),
        ("{} sessions", ""),
        ("scam_title", ""),
        ("scam_text1", ""),
        ("scam_text2", ""),
        ("Don't show again", ""),
        ("I Agree", ""),
        ("Decline", ""),
        ("Timeout in minutes", ""),
        ("auto_disconnect_option_tip", ""),
        ("Connection failed due to inactivity", ""),
        ("Check for software update on startup", ""),
        ("upgrade_rustdesk_server_pro_to_{}_tip", ""),
        ("pull_group_failed_tip", ""),
        ("Filter by intersection", ""),
        ("Remove wallpaper during incoming sessions", ""),
        ("Test", ""),
        ("display_is_plugged_out_msg", ""),
        ("No displays", ""),
        ("Open in new window", ""),
        ("Show displays as individual windows", ""),
        ("Use all my displays for the remote session", ""),
        ("selinux_tip", ""),
        ("Change view", ""),
        ("Big tiles", ""),
        ("Small tiles", ""),
        ("List", ""),
        ("Virtual display", ""),
        ("Plug out all", ""),
        ("True color (4:4:4)", ""),
        ("Enable blocking user input", ""),
        ("id_input_tip", ""),
        ("privacy_mode_impl_mag_tip", ""),
        ("privacy_mode_impl_virtual_display_tip", ""),
        ("Enter privacy mode", ""),
        ("Exit privacy mode", ""),
        ("idd_not_support_under_win10_2004_tip", ""),
        ("input_source_1_tip", ""),
        ("input_source_2_tip", ""),
        ("Swap control-command key", ""),
        ("swap-left-right-mouse", ""),
        ("2FA code", ""),
        ("More", ""),
        ("enable-2fa-title", ""),
        ("enable-2fa-desc", ""),
        ("wrong-2fa-code", ""),
        ("enter-2fa-title", ""),
        ("Email verification code must be 6 characters.", ""),
        ("2FA code must be 6 digits.", ""),
        ("Multiple Windows sessions found", ""),
        ("Please select the session you want to connect to", ""),
        ("powered_by_me", ""),
        ("outgoing_only_desk_tip", ""),
        ("preset_password_warning", ""),
        ("Security Alert", ""),
        ("My address book", ""),
        ("Personal", ""),
        ("Owner", ""),
        ("Set shared password", ""),
        ("Exist in", ""),
        ("Read-only", ""),
        ("Read/Write", ""),
        ("Full Control", ""),
        ("share_warning_tip", ""),
        ("Everyone", ""),
        ("ab_web_console_tip", ""),
        ("allow-only-conn-window-open-tip", ""),
        ("no_need_privacy_mode_no_physical_displays_tip", ""),
        ("Follow remote cursor", ""),
        ("Follow remote window focus", ""),
        ("default_proxy_tip", ""),
        ("no_audio_input_device_tip", ""),
        ("Incoming", ""),
        ("Outgoing", ""),
        ("Clear Wayland screen selection", ""),
        ("clear_Wayland_screen_selection_tip", ""),
        ("confirm_clear_Wayland_screen_selection_tip", ""),
        ("android_new_voice_call_tip", ""),
        ("texture_render_tip", ""),
        ("Use texture rendering", ""),
        ("Floating window", ""),
        ("floating_window_tip", ""),
        ("Keep screen on", ""),
        ("Never", ""),
        ("During controlled", ""),
        ("During service is on", ""),
        ("Capture screen using DirectX", ""),
        ("Back", ""),
        ("Apps", ""),
        ("Volume up", ""),
        ("Volume down", ""),
        ("Power", ""),
        ("Telegram bot", ""),
        ("enable-bot-tip", ""),
        ("enable-bot-desc", ""),
        ("cancel-2fa-confirm-tip", ""),
        ("cancel-bot-confirm-tip", ""),
        ("About LongShengLink", ""),
        ("Send clipboard keystrokes", ""),
        ("network_error_tip", ""),
        ("Unlock with PIN", ""),
        ("Requires at least {} characters", ""),
        ("Wrong PIN", ""),
        ("Set PIN", ""),
        ("Enable trusted devices", ""),
        ("Manage trusted devices", ""),
        ("Platform", ""),
        ("Days remaining", ""),
        ("enable-trusted-devices-tip", ""),
        ("Parent directory", ""),
        ("Resume", ""),
        ("Invalid file name", ""),
        ("one-way-file-transfer-tip", ""),
        ("Authentication Required", ""),
        ("Authenticate", ""),
        ("web_id_input_tip", ""),
        ("Download", ""),
        ("Upload folder", ""),
        ("Upload files", ""),
        ("Clipboard is synchronized", ""),
    ].iter().cloned().collect();
}
