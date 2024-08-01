Overview

The rust program in json_aggregate was used to proccess scraped json and pdfs into xml files and eventually send requests to the api to post everything.

The JavaScript scripts in scraper were used to scrape the web for json using a library called puppeteer

The proccess of transfer required direct ssh access. The xml files were compressed and uploaded through scp

The files were then submitted to OPS with the included php xml import script in htdocs/ops/tools

After the files were submitted, the rust program sent requests to the OPS api, which resulted in the posting of all the files.
