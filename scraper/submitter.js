const puppeteer = require('puppeteer');
const fs = require('fs');
const shell = require('shelljs');
function delay(time) {
   return new Promise(function(resolve) { 
       setTimeout(resolve, time)
   });
}

let i = 1;
console.log("Reading "+i);

  let files_list = [];
  fs.readdir("../json_agregate/xml/xml"+i, (err, files) => {
    files.forEach(file => {
    var stats = fs.statSync("../json_agregate/xml/xml"+i+"/"+file)
    var filesize = stats.size;
    let f = [];
    if (filesize/(1024*1024)>40){
      console.log("File too big.. skipping");
    }else{
      files_list.push("../json_agregate/xml/xml"+i+"/"+file);
    }
    });
  });

(async () => {
  console.log("Starting...");
  let start_page = 1;
  let username = "admin";
  let password = "princeton";


  // Launch the browser
  const browser = await puppeteer.launch({
    headless: true, args: ['--start-maximized',
      `--disable-blink-features=AutomationControlled`,
      `--disable-web-security`,
      `--allow-running-insecure-content`],
    defaultViewport: null
  });

 let page = await browser.newPage();
  await page.setUserAgent(
    'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/80.0.3987.163 Safari/537.36'
  );


  // Go to your site
  await page.goto('https://ayurxiv.org/index.php/server/management/importexport/plugin/NativeImportExportPlugin');
  await page.locator('#username').fill(username);
  await page.locator('#password').fill(password);
  await page.locator('.submit').click();
  await page.waitForNavigation({WaitForOptions:'load'});
  // await delay(5000);
  let pages = [];
  pages.push(page);
  // for(var i = 0; i<4; i++){
  //   let newpage = await browser.newPage();
  //   await newpage.goto('https://ayurxiv.org/index.php/server/management/importexport/plugin/NativeImportExportPlugin');
  //   if(i==0){
  //     // await page.locator('#username').fill(username);
  //     await newpage.locator('#password').fill(password);
  //     await newpage.locator('.submit').click();
  //   }
  //   // await newpage.waitForNavigation({WaitForOptions:'load'});
  //   pages.push(newpage);
  //   
  // }
  console.log("Uploading "+files_list.length+" Files");
  let index = 0;

  while(index<files_list.length){
  for(var pg of pages){
    const fileElement = await pg.waitForSelector('input[type=file]');
      console.log("Found upload element");
      try{
        await fileElement.uploadFile(files_list[index]);
      }catch(e){
        console.log(e)
        continue;
      }
      // const error = await pg.evaluate(()=>{
      //   document.get
      // });
    index++;
  }
  //await file uploads
    console.log("Waiting for upload");
    console.log("Uploaded");
    // await delay(1000);
    let subtracting = false;
  for(var pg of pages){
      try{
        await pg.locator('.pkpUploaderFilename').filter(button=>button.innerText.includes("PMC")).wait();
      }catch(e){
        console.log(e);
        subtracting = true;
        
        // index--;
      }
    }
  promises = [];
  for(var pg of pages){
    promises.push(pg.locator(".pkp_button.submitFormButton").click());
  }
console.log("Submitted");
  for (var p of promises){
      console.log("Clicking button");
      try{
        await p;
      }catch(e){
        console.log(e);
        console.log("Button not found");
        subtracting = true;
        // index--;
      }
    }
  promises = [];
// console.log("Waiting");
    // await delay(1000);
console.log("returning");
  for(var pg of pages){
    // promises.push(pg.locator(".close").wait());
    promises.push(pg.locator("#ui-id-1").click());
  }
    console.log("Waiting again");
  for (var p of promises){
      try{

      await p;
      }catch(e){
        console.log(e)
        console.log("button not found...");
      }
    }
    console.log("done");
    if(subtracting){
      index--;
    }
  for(var pg of pages){
      try{
        await pg.goto('https://ayurxiv.org/index.php/server/management/importexport/plugin/NativeImportExportPlugin');
      }catch(e){
        console.log(e);
      }
    // await page.locator('#username').fill(username);
    // await pg.locator('#password').fill(password);
    // await pg.locator('.submit').click();
    // await pg.waitForNavigation({WaitForOptions:'load'});
    }

}

})();


