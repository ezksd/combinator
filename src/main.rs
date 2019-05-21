use scheme_rust::json::*;
use scheme_rust::parser::*;
fn main() {
    let x = json_value().parse(r#"{"proxies":{"DIRECT":{"history":[],"type":"Direct"}
    ,"GLOBAL":{"all":["DIRECT","REJECT","湖南中转无限速(0.5倍率) -- 1","江苏中转无限速(0.5倍率) -- 1",
    "江苏中转无限速(0.5倍率) -- 2","深港IPLC无限速(1倍率) -- 1","深港IPLC无限速(2倍率) -- 2","深港IPLC无限速(2倍率) -- 3"
    ,"深港IPLC无限速(2倍率) -- 4","沪港IPLC无限速(2倍率) -- 1","沪港IPLC无限速(2倍率) -- 2",
    "杭港IPLC无限速(2倍率) -- 1","杭港IPLC无限速(2倍率) -- 2","杭港IPLC无限速(1倍率) -- 3","杭港IPLC无限速(1倍率) 
    -- 4","日本阿里云BBTEC(1倍率) -- 1","日本阿里云BBTEC(1倍率) -- 2","香港阿里云限速(1倍率) -- 1","香港阿里云限速(1倍率) -- 2","香港阿里云限速(1倍率) -- 3",
    "香港HKBN(0.5倍率) -- 1","香港HKBN(0.5倍率) -- 2","香港HKT(0.5倍率) -- 1",
    "香港HKT(0.5倍率) -- 2","香港HKT(0.5倍率) -- 3","香港HKT(0.5倍率) -- 4",
    "香港HKT(0.5倍率) -- 5","香港HKT(0.5倍率) -- 6","香港WTT(0.5倍率) -- 1","香港WTT(0.5倍率) -- 2","香港CN2(1倍率) -- 1","新加坡(0.5倍率) -- 1","新加坡(0.5倍率) -- 2","澳门BGP(0.5倍率) -- 1","台湾HINET(0.5倍率) -- 1","台湾HINET(0.5倍率) -- 2","日本IDCF无限速(0.5倍率) -- 1","日本IDCF无限速(0.5倍率) -- 2","日本软银无限速(1.5倍率) -- 1","日本软银无限速(1.5倍率) -- 2","日本软银无限速(1.5倍率) -- 3","日本软银无限速(1.5倍率) -- 4","韩国SK(3倍率) -- 1","美国GIA CN2(0.5倍率) -- 1","美国GIA CN2(0.5倍率) -- 2","美国GIA CN2(0.5倍率) -- 3","美国GIA CN2(0.5倍率) -- 4","美国GIA CN2(0.5倍率) -- 5","美国GIA CN2(0.5倍率) -- 6","美国GIA CN2(0.5倍率) -- 7","美国GIA CN2(0.5倍率) -- 8","德国CN2(1倍率) -- 1","Proxy"],"history":[],"now":"DIRECT","type":"Selector"},"Proxy":{"all":["湖南中转无限速(0.5倍率) -- 1","江苏中转无限速(0.5倍率) -- 1","江苏中转无限速(0.5倍率) -- 2","深港IPLC无限速(2倍率) -- 2","深港IPLC无限速(1倍率) -- 1","深港IPLC无限速(2倍率) -- 3","深港IPLC无限速(2倍率) -- 4","沪港IPLC无限速(2倍率) -- 1","沪港IPLC无限速(2倍率) -- 2","杭港IPLC无限速(2倍率) -- 1","杭港IPLC无限速(2倍率) -- 2","杭港IPLC无限速(1倍率) -- 3","杭港IPLC无限速(1倍率) -- 4","日本阿里云BBTEC(1倍率) -- 1","日本阿里云BBTEC(1倍率) -- 2","台湾HINET(0.5倍率) -- 1","台湾HINET(0.5倍率) -- 2","美国GIA CN2(0.5倍率) -- 1","美国GIA CN2(0.5倍率) -- 2","美国GIA CN2(0.5倍率) -- 3","美国GIA CN2(0.5倍率) -- 4","美国GIA CN2(0.5倍率) -- 5","美国GIA CN2(0.5倍率) -- 6","美国GIA CN2(0.5倍率) -- 7","美国GIA CN2(0.5倍率) -- 8"],"history":[],"now":"日本阿里云BBTEC(1倍率) -- 1","type":"Selector"},"REJECT":{"history":[],"type":"Reject"},"台湾HINET(0.5倍率) -- 1":{"history":[{"time":"2019-05-20T16:35:54.64319393+08:00","delay":150}],"type":"Shadowsocks"},"台湾HINET(0.5倍率) -- 2":{"history":[{"time":"2019-05-20T16:35:54.967554218+08:00","delay":343}],"type":"Shadowsocks"},"德国CN2(1倍率) -- 1":{"history":[{"time":"2019-05-20T16:35:57.377519278+08:00","delay":775}],"type":"Shadowsocks"},"新加坡(0.5倍率) -- 1":{"history":[{"time":"2019-05-20T16:35:54.282666874+08:00","delay":168}],"type":"Shadowsocks"},"新加坡(0.5倍率) -- 2":{"history":[{"time":"2019-05-20T16:35:54.830347462+08:00","delay":685}],"type":"Shadowsocks"},"日本IDCF无限速(0.5倍率) -- 1":{"history":[{"time":"2019-05-20T16:35:55.244579201+08:00","delay":600}],"type":"Shadowsocks"},"日本IDCF无限速(0.5倍率) -- 2":{"history":[{"time":"2019-05-20T16:35:55.000583142+08:00","delay":168}],"type":"Shadowsocks"},"日本软银无限速(1.5倍率) -- 1":{"history":[{"time":"2019-05-20T16:35:55.111154865+08:00","delay":142}],"type":"Shadowsocks"},"日本软银无限速(1.5倍率) -- 2":{"history":[{"time":"2019-05-20T16:35:55.134836119+08:00","delay":133}],"type":"Shadowsocks"},"日本软银无限速(1.5倍率) -- 3":{"history":[{"time":"2019-05-20T16:35:56.094535806+08:00","delay":982}],"type":"Shadowsocks"},"日本软银无限速(1.5倍率) -- 4":{"history":[{"time":"2019-05-20T16:35:55.35106908+08:00","delay":227}],"type":"Shadowsocks"},"日本阿里云BBTEC(1倍率) -- 1":{"history":[{"time":"2019-05-20T16:35:51.65200684+08:00","delay":642}],"type":"Shadowsocks"},"日本阿里云BBTEC(1倍率) -- 2":{"history":[{"time":"2019-05-20T16:35:51.493244182+08:00","delay":444}],"type":"Shadowsocks"},"杭港IPLC无限速(1倍率) -- 3":{"history":[{"time":"2019-05-20T16:35:50.509779513+08:00","delay":67}],"type":"Shadowsocks"},"杭港IPLC无限速(1倍率) -- 4":{"history":[{"time":"2019-05-20T16:35:51.008263934+08:00","delay":497}],"type":"Shadowsocks"},"杭港IPLC无限速(2倍率) -- 1":{"history":[{"time":"2019-05-20T16:35:51.37119954+08:00","delay":1345}],"type":"Shadowsocks"},"杭港IPLC无限速(2倍率) -- 2":{"history":[{"time":"2019-05-20T16:35:51.046981723+08:00","delay":849}],"type":"Shadowsocks"},"江苏中转无限速(0.5倍率) -- 1":{"history":[{"time":"2019-05-20T16:35:49.600550186+08:00","delay":911}],"type":"Shadowsocks"},"江苏中转无限速(0.5倍率) -- 2":{"history":[{"time":"2019-05-20T16:35:50.43973574+08:00","delay":1750}],"type":"Shadowsocks"},"沪港IPLC无限速(2倍率) -- 1":{"history":[{"time":"2019-05-20T16:35:50.023177951+08:00","delay":712}],"type":"Shadowsocks"},"沪港IPLC无限速(2倍率) -- 2":{"history":[{"time":"2019-05-20T16:35:51.503265931+08:00","delay":1901}],"type":"Shadowsocks"},"深港IPLC无限速(1倍率) -- 1":{"history":[{"time":"2019-05-20T16:35:48.767529834+08:00","delay":74}],"type":"Shadowsocks"},"深港IPLC无限速(2倍率) -- 2":{"history":[{"time":"2019-05-20T16:35:50.196440685+08:00","delay":1504}],"type":"Shadowsocks"},"深港IPLC无限速(2倍率) -- 3":{"history":[{"time":"2019-05-20T16:35:49.308599415+08:00","delay":539}],"type":"Shadowsocks"},"深港IPLC无限速(2倍率) -- 4":{"history":[{"time":"2019-05-20T16:35:54.448091911+08:00","delay":5636},{"time":"2019-05-20T16:35:57.906681337+08:00","delay":1205}],"type":"Shadowsocks"},"湖南中转无限速(0.5倍率) -- 1":{"history":[{"time":"2019-05-20T16:35:48.810226627+08:00","delay":121}],"type":"Shadowsocks"},"澳门BGP(0.5倍率) -- 1":{"history":[{"time":"2019-05-20T16:35:55.122102524+08:00","delay":838}],"type":"Shadowsocks"},"美国GIA CN2(0.5倍率) -- 1":{"history":[{"time":"2019-05-20T16:35:55.961488925+08:00","delay":715}],"type":"Shadowsocks"},"美国GIA CN2(0.5倍率) -- 2":{"history":[{"time":"2019-05-20T16:35:56.265106912+08:00","delay":912}],"type":"Shadowsocks"},"美国GIA CN2(0.5倍率) -- 3":{"history":[{"time":"2019-05-20T16:35:56.700099596+08:00","delay":938}],"type":"Shadowsocks"},"美国GIA CN2(0.5倍率) -- 4":{"history":[{"time":"2019-05-20T16:35:56.2570179+08:00","delay":294}],"type":"Shadowsocks"},"美国GIA CN2(0.5倍率) -- 5":{"history":[{"time":"2019-05-20T16:35:56.38321745+08:00","delay":287}],"type":"Shadowsocks"},"美国GIA CN2(0.5倍率) -- 6":{"history":[{"time":"2019-05-20T16:35:56.600815181+08:00","delay":342}],"type":"Shadowsocks"},"美国GIA CN2(0.5倍率) -- 7":{"history":[{"time":"2019-05-20T16:35:57.105905169+08:00","delay":839}],"type":"Shadowsocks"},"美国GIA CN2(0.5倍率) -- 8":{"history":[{"time":"2019-05-20T16:35:56.988804843+08:00","delay":604}],"type":"Shadowsocks"},"韩国SK(3倍率) -- 1":{"history":[{"time":"2019-05-20T16:35:55.760058948+08:00","delay":623}],"type":"Shadowsocks"},"香港CN2(1倍率) -- 1":{"history":[{"time":"2019-05-20T16:35:54.112514396+08:00","delay":94}],"type":"Shadowsocks"},"香港HKBN(0.5倍率) -- 1":{"history":[{"time":"2019-05-20T16:35:52.893361916+08:00","delay":1386}],"type":"Shadowsocks"},"香港HKBN(0.5倍率) -- 2":{"history":[{"time":"2019-05-20T16:35:53.787145298+08:00","delay":2203}],"type":"Shadowsocks"},"香港HKT(0.5倍率) -- 1":{"history":[{"time":"2019-05-20T16:35:52.22052334+08:00","delay":567}],"type":"Shadowsocks"},"香港HKT(0.5倍率) -- 2":{"history":[{"time":"2019-05-20T16:35:52.044862524+08:00","delay":181}],"type":"Shadowsocks"},"香港HKT(0.5倍率) -- 3":{"history":[{"time":"2019-05-20T16:35:57.046112434+08:00","delay":0},{"time":"2019-05-20T16:36:02.046658627+08:00","delay":0},{"time":"2019-05-20T16:36:07.047243267+08:00","delay":0},{"time":"2019-05-20T16:36:12.047931216+08:00","delay":0},{"time":"2019-05-20T16:36:17.048399302+08:00","delay":0},{"time":"2019-05-20T16:36:22.0489131+08:00","delay":0}],"type":"Shadowsocks"},"香港HKT(0.5倍率) -- 4":{"history":[{"time":"2019-05-20T16:35:53.824108755+08:00","delay":1602}],"type":"Shadowsocks"},"香港HKT(0.5倍率) -- 5":{"history":[{"time":"2019-05-20T16:35:54.017253014+08:00","delay":0}],"type":"Shadowsocks"},"香港HKT(0.5倍率) -- 6":{"history":[{"time":"2019-05-20T16:35:54.491706097+08:00","delay":703}],"type":"Shadowsocks"},"香港WTT(0.5倍率) -- 1":{"history":[{"time":"2019-05-20T16:35:54.14376056+08:00","delay":0}],"type":"Shadowsocks"},"香港WTT(0.5倍率) -- 2":{"history":[{"time":"2019-05-20T16:35:54.62252278+08:00","delay":797}],"type":"Shadowsocks"},"香港阿里云限速(1倍率) -- 1":{"history":[{"time":"2019-05-20T16:35:51.459269865+08:00","delay":86}],"type":"Shadowsocks"},"香港阿里云限速(1倍率) -- 2":{"history":[{"time":"2019-05-20T16:35:51.862179963+08:00","delay":401}],"type":"Shadowsocks"},"香港阿里云限速(1倍率) -- 3":{"history":[{"time":"2019-05-20T16:35:51.582246224+08:00","delay":87}],"type":"Shadowsocks"}}}"#);
    print!("{:?}", x)
    // print!("123")
}
