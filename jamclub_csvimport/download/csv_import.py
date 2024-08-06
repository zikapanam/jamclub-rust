import os
from pyairtable import Api
from dotenv import load_dotenv
import csv
from datetime import datetime
from dateutil import parser
import pytz
timezone = pytz.timezone("Europe/Paris")

def new_dict(fields_dict, fieldnames):
    result = {}
    for fieldname in fieldnames:
        result[fieldname] = returnValue(fields_dict, fieldname)

    return result

def returnValue(fields_dict, fieldname, type='str'):

    try:
        try:
            strdate = fields_dict[fieldname]
            date = datetime.strptime(strdate, "%Y-%m-%dT%H:%M:%S.%f%z")
            
            result = date.astimezone(timezone).isoformat()            
            return result
        except Exception as e:
            pass

        if isinstance(fields_dict[fieldname], list):
            return ",".join(fields_dict[fieldname])
        elif isinstance(fields_dict[fieldname], str):
            return fields_dict[fieldname].strip()
        else:
            return fields_dict[fieldname]
    except:
        if type == 'str':
            return ""
        elif type == 'list':
            return []
        else:
            return None

def main():
    load_dotenv()
    import_date = None
    
    # MEMBRES
    airtable_api = Api(os.getenv('AIRTABLE_API_TOKEN'))
    table = airtable_api.table('appP8kY2qxO1uwfUl', 'tbl1Ory2Sd3WE7xgs')
    records = []
    if import_date:
        records = table.all(view="CsvFile", formula='{Created} >= "' + import_date.isoformat() + '"', max_records=1000)
    else:
        records = table.all(view="CsvFile", max_records=1000)
    
    memb_fieldnames = ('Created', "Dernière modification",'Roles PJC', 'Roles MIP', "User ID Discord", "Pseudo Discord", "Email", \
                'Pseudo ZAP', "Prénom", "Nom", "Téléphone", "Adresse", "Code postal", \
                "Présentation", "Remarques", "Provenance", "Diffusion audio", "Diffusion Vidéo", "Newsletter ?", "Notes", \
                 "passdroits", "Membre ZAP RECORD_ID", "Role(s) Communauté(s)", "UUID", "Type(s) de rencontre", \
                "strCommunautés", "strInstruments", "strStylesDeMusique", "Cotisant ?", "Date de naissance", "Statut")
    1
    with open('membres.csv', 'w') as csvfilewrite:
        writer = csv.DictWriter(csvfilewrite, fieldnames=memb_fieldnames, delimiter=';',
                            quotechar='"', quoting=csv.QUOTE_ALL)
        writer.writeheader()
        for record in records:
            cols = new_dict(record['fields'], memb_fieldnames)
            writer.writerow(cols)

    # LIEUX
    airtable_api = Api(os.getenv('AIRTABLE_API_TOKEN'))
    table = airtable_api.table('appP8kY2qxO1uwfUl', 'tblgvkkeJ7JLU1swS')
    records = []
    if import_date:
        records = table.all(view="CsvFile", formula='{Created time} >= "' + import_date.isoformat() + '"', max_records=1000)
    else:
        records = table.all(view="CsvFile", max_records=1000)


    lieux_fieldnames = { "RECORD_ID", "Intitulé court", "Intitulé long", "Adresse", "Code postal", "Type de Lieu", 
                        "Google Maps URL", "Téléphone", "Email", "Notes", "Latitude", "Longitude","Archivé", "Created"}


    with open('lieux.csv', 'w') as csvfilewrite:
        writer = csv.DictWriter(csvfilewrite, fieldnames=lieux_fieldnames, delimiter=';',
                            quotechar='"', quoting=csv.QUOTE_ALL)
        writer.writeheader()
        for record in records:
            if (record['fields']['Intitulé long'] != "Cf. Description"):
                cols = new_dict(record['fields'], lieux_fieldnames)
                writer.writerow(cols)



    # COLLECTIFS
    airtable_api = Api(os.getenv('AIRTABLE_API_TOKEN'))
    table = airtable_api.table('appP8kY2qxO1uwfUl', 'tblBA1DUU5uED0NtM')
    records = []
    if import_date:
        records = table.all(view="CsvFile", formula='{Created time} >= "' + import_date.isoformat() + '"', max_records=1000)
    else:
        records = table.all(view="CsvFile", max_records=1000)


    coll_fieldnames = { "RECORD_ID", "Intitulé", "intitulé court", "DISCORD RoleID", "DISCORD ChannelID", "LISTE Inscrits", 
                   "strCommunauté", "REF Responsable", "Notes", "LISTE Référents lineup","Jam description",
                   "Archivé","discord_presentation_url","Creation time","Last modified time"}

    with open('collectifs.csv', 'w') as csvfilewrite:
        writer = csv.DictWriter(csvfilewrite, fieldnames=coll_fieldnames, delimiter=';',
                            quotechar='"', quoting=csv.QUOTE_ALL)
        writer.writeheader()
        for record in records:
            cols = new_dict(record['fields'], coll_fieldnames)
            writer.writerow(cols)

    # RENCONTRES
    airtable_api = Api(os.getenv('AIRTABLE_API_TOKEN'))
    table = airtable_api.table('appP8kY2qxO1uwfUl', 'tblBI7FN41iwa6TIY')
    records = []
    if import_date:
        records = table.all(view="CsvFile", formula='{Created time} >= "' + import_date.isoformat() + '"', max_records=1000)
    else:
        records = table.all(view="CsvFile", max_records=1000)

    renc_fieldnames = {"REF Organisateur", "Intitulé", "Description","Date de Rencontre","Durée","REF Lieu","REF Collectif",
    "LISTE Participants","LISTE Communautés","Notes","Type de rencontre","DISCORD Message URL",
    "Statut","DISCORD MessageID","DISCORD ThreadID","Last modified time","Created time","Instrument(s) recherché(s)",
    "Participants Non dispo","Participants incertain (ou disponible si beoin)", "Liste des absents","Rappel 14 jours avant fait ?",
    "Rappel 7 jours avant fait ?","Rappel 2 heures avant fait ?","DISCORD ChannelID","Intitulé long","Fin Rencontre","Duration",
    "strInstrumentsRecherches","strCommunautés", "RECORD_ID", "Styles de musique", "Inscription ouverte jusqu'au dernier moment"}

    with open('rencontres.csv', 'w') as csvfilewrite:
        writer = csv.DictWriter(csvfilewrite, fieldnames=renc_fieldnames, delimiter=';',
                            quotechar='"', quoting=csv.QUOTE_ALL)
        writer.writeheader()
        for record in records:
            cols = new_dict(record['fields'], renc_fieldnames)
            writer.writerow(cols)

    # EVENTS
    airtable_api = Api(os.getenv('AIRTABLE_API_TOKEN'))
    table = airtable_api.table('appP8kY2qxO1uwfUl', 'tblT9GF8Q73TB1Ndv')
    records = []
    if import_date:
        records = table.all(view="CsvFile", formula='{Created time} >= "' + import_date.isoformat() + '"', max_records=1000)
    else:
        records = table.all(view="CsvFile", max_records=1000)


    event_fieldnames = { "RECORD_ID", "LISTE_Organisateurs", "Intitulé", "Type", "Description", "Statut", "REF_Lieu", 
                        "Date", "Durée", "Notes", "Lien Mobilizon" , "Created", "Modified", "Bénévoles", "LISTE Lineups"}

    with open('evenements.csv', 'w') as csvfilewrite:
        writer = csv.DictWriter(csvfilewrite, fieldnames=event_fieldnames, delimiter=';',
                            quotechar='"', quoting=csv.QUOTE_ALL)
        writer.writeheader()
        for record in records:
            cols = new_dict(record['fields'], event_fieldnames)
            writer.writerow(cols)

    # LINEUPS
    airtable_api = Api(os.getenv('AIRTABLE_API_TOKEN'))
    table = airtable_api.table('appP8kY2qxO1uwfUl', 'tblMWJFbl2qr8B5eV')
    records = []
    if import_date:
        records = table.all(view="CsvFile", formula='{Created time} >= "' + import_date.isoformat() + '"', max_records=1000)
    else:
        records = table.all(view="CsvFile", max_records=1000)


    line_fieldnames = { "RECORD_ID", "intitulé court", "intitulé long", "Statut", "REF Référent","Membres","REF Collectif","Style(s) de musique",
                       "Phrase d'accroche","Description","Événements Musicaux", "Created", "Modified"}

    with open('lineups.csv', 'w') as csvfilewrite:
        writer = csv.DictWriter(csvfilewrite, fieldnames=line_fieldnames, delimiter=';',
                            quotechar='"', quoting=csv.QUOTE_ALL)
        writer.writeheader()
        for record in records:
            cols = new_dict(record['fields'], line_fieldnames)
            writer.writerow(cols)


    # BOTS
    airtable_api = Api(os.getenv('AIRTABLE_API_TOKEN'))
    table = airtable_api.table('appP8kY2qxO1uwfUl', 'tblpCbTFYQ6GdFNeI')
    records = []
    if import_date:
        records = table.all(view="CsvFile", formula='{Created time} >= "' + import_date.isoformat() + '"', max_records=1000)
    else:
        records = table.all(view="CsvFile", max_records=1000)


    bot_fieldnames = { "Bot Name", "Last succesfull execution time","Description","RecordId"}

    with open('bots.csv', 'w') as csvfilewrite:
        writer = csv.DictWriter(csvfilewrite, fieldnames=bot_fieldnames, delimiter=';',
                            quotechar='"', quoting=csv.QUOTE_ALL)
        writer.writeheader()
        for record in records:
            cols = new_dict(record['fields'], bot_fieldnames)
            writer.writerow(cols)



if __name__ == "__main__":
    main()