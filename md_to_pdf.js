const fs = require('fs');
const readline = require('readline');
const markdownIt = require('markdown-it');
const markdownItMermaid = require('markdown-it-mermaid').default;
const puppeteer = require('puppeteer');

const md = markdownIt();
md.use(markdownItMermaid);

const rl = readline.createInterface({
  input: process.stdin,
  output: process.stdout
});

rl.question('Please enter the input markdown file name: ', function(inputFileName) {
  // Output file name is same as input file name with .pdf extension
  const outputFileName = inputFileName.split('.').slice(0, -1).join('.') + '.pdf';

  // MarkdownをHTMLに変換（マーメイドも変換されます）
  const markdown = fs.readFileSync(inputFileName, 'utf8');
  const html = md.render(markdown);

  // HTMLをPDFに変換
  (async () => {
    const browser = await puppeteer.launch();
    const page = await browser.newPage();
    await page.setContent(html, { waitUntil: 'networkidle0' });
    await page.pdf({ path: outputFileName, format: 'A4' });
    await browser.close();
  })();

  rl.close();
});
