struct Pagina{
    title:Option<String>,
    precio:Option<String>
}


fn main() {
    let url = reqwest::blocking::get("https://dolar.wilkinsonpc.com.co/divisas/");

    let html = url.unwrap().text().unwrap();

    println!("{html}");

    let doc_parse = scraper::Html::parse_document(&html);

    let html_selector = scraper::Selector::parse("td.historico").unwrap();
    
    let html_seleccionar = doc_parse.select(&html_selector);

    let mut monedas:Vec<Pagina>=Vec::new();

    for Html_Seleccionar in html_seleccionar{
        let url = Html_Seleccionar
            .select(&scraper::Selector::parse("a").unwrap())
            .next()
            .and_then(|a| a.value().attr("href"))
            .map(str::to_owned);

        let precio = Html_Seleccionar
            .select(&scraper::Selector::parse(".ultimo cierre").unwrap())
            .next()
            .map(|price| price.text().collect::<String>());
        let pagina = Pagina { title: url, precio: precio };
        monedas.push(pagina);
    
    }


    // create the CSV output file
    let path = std::path::Path::new("monedas.csv");
    let mut writer = csv::Writer::from_path(path).unwrap();

    // append the header to the CSV
    writer
        .write_record(&["url","price"])
        .unwrap();
    // populate the output file
    for product in monedas {
        //let url = product.url.unwrap();
        let url = product.title.unwrap();
        let precio = product.precio.unwrap();
        //let price = product.price.unwrap();
        writer.write_record(&[url, precio]).unwrap();
    }

    // free up the resources
    writer.flush().unwrap();
    
}
