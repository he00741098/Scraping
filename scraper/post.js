const puppeteer = require('puppeteer');
function delay(time) {
 return new Promise(function(resolve) { 
  setTimeout(resolve, time)
 });
}

(async () => {
 console.log("Starting...");
 let start_page = 1;
 let username = "admin";
 let password = "princeton";


 // Launch the browser
 const browser = await puppeteer.launch({
  headless: false, args: ['--start-maximized',
   `--disable-blink-features=AutomationControlled`,
   `--disable-web-security`,
   `--allow-running-insecure-content`],
  defaultViewport: null
 });

 let pager = await browser.newPage();
 await pager.setUserAgent(
  'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/80.0.3987.163 Safari/537.36'
 );
 let pages = []
 for (var i=0; i<5;i++){
  let page = await browser.newPage();
  await page.setUserAgent(
   'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/80.0.3987.163 Safari/537.36'
  );
  pages.push(page);
 }

 // Go to your site
 await pager.bringToFront();
 await pager.goto('https://ayurxiv.org/index.php/server/submissions');
 for(page of pages){
  await page.goto('https://ayurxiv.org/index.php/server/submissions');
 }
 console.log("Starting servants");
 let pageLoad = [];
 for(page of pages){
  await page.bringToFront();
  await page.locator('#username').fill(username);
  await page.locator('#password').fill(password);
  await page.locator('.submit').click();
  pageLoad.push(page.waitForNavigation({WaitForOptions:'load'}));
 }
 console.log("Filled In");
 await pager.bringToFront();
 await pager.locator('#username').fill(username);
 await pager.locator('#password').fill(password);
 await pager.locator('.submit').click();
 await pager.waitForNavigation({WaitForOptions:'load'});
 for(p of pageLoad){
  await p;
 }
 console.log("logged in");

 pager.bringToFront();
 await pager.goto('https://ayurxiv.org/index.php/server/submissions#active');
 big_loop: while(true){
  try{
   pager.bringToFront();
   await pager.goto('https://ayurxiv.org/index.php/server/submissions#active');
   // await page.evaluate(() => {
   //  location.reload(true)
   // })
   // await pager.waitForNavigation({WaitForOptions:'load'});
   // await page.reload({ waitUntil: ["domcontentloaded"] });
   await pager.locator(".pkpBadge.pkpBadge--button.listPanel__item--submission__stage.pkpBadge--dot.pkpBadge--production").wait();
   console.log("Badges loaded");
   let links = await pager.evaluate(()=>{
    let array = [];
    let thing =0;
    for(g of document.querySelectorAll("a.pkpButton")){
     if(thing>29){
      break;
     }
     if(g.href.includes("/access")){
      thing+=1
      array.push(g.href+"/5#publication")
     }
    }
    return array;
   });

   while (links.length>0){
   console.log(links.length);
    let promises = [];
    let seen = [];
    for(page of pages){
     // await delay(1000);
     if (links.length>0){
      let link = links.pop();
      while (links.length>0&&seen.includes(link)){
       link = links.pop()
      }
      if(links.length==0){
       continue big_loop;
      }
      promises.push(page.goto(link));
     }
    }
    for(p of promises){
     await p;
    }
    console.log("Navigated!");
    promises = [];
    for(page of pages){
     promises.push(page.waitForSelector("#license-button", {visible:true}));
    }
    for(p of promises){
     await p;
    }
    console.log("license button found");
    // await page.locator(".pkpTable").wait();
    // await delay(1000);
    // await page.screenshot({path: 'test.png'});
    promises = [];
    for(page of pages){
     await page.bringToFront();
     try{
      await page.waitForFunction("document.querySelectorAll('.pkpButton')[5].innerText=='Post'");
      await page.evaluate(()=>document.querySelectorAll('.pkpButton')[5].click());
      await page.waitForFunction("document.querySelectorAll('[label=Post]').length==1");
      await page.evaluate(()=>document.querySelectorAll('[label=Post]')[0].click());
      promises.push(page.waitForSelector(".pkpPublication__versionPublished", {visible:true}));
     }catch(e){
      console.log("No post button...");
     }
    }
    for (p of promises){
     await p;
    }

    console.log("Done with some");
   }
   console.log("Done with page");

  }catch(e){ console.log(e);
  }
 }
})();


