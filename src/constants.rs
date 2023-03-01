pub const SPLASH: &str = r#"
   _______________ 
  / __/ __/ __/ _ |
 / _/_\ \_\ \/ __ |
/_/ /___/___/_/ |_|
                   
» Now open-source!
"#;

/// Called when the horse market has been initalized.
pub const GAME_HMARKET_START: &str = "676C6F62616C2F486F7273654D61726B6574506C6163652F486F72736573466F7253616C652F486F727365732F47656E322F456E676C69736854686F726F75676862726564446973706C61792E476C6F62616C41636365737353686F72746375742822486F7273654D61726B6574486F727365466F7253616C655F303122293B0D0A";

// Called when the player presses on the inventory button in the UI.
pub const GAME_INV_CLICK: &str = "2F2F676C6F62616C2F536F756E644D697865722E536F756E644D69786572506C617928224755495F436C69636B31222C362C30293B0D0A0D0A69662820676C6F62616C2F496E76656E746F727957696E646F772E476574416374696F6E53746172746564282920290D0A7B0D0A09676C6F62616C2F496E76656E746F727957696E646F772E53746F7028293B0D0A7D0D0A656C73650D0A7B0D0A09676C6F62616C2F496E76656E746F727957696E646F772E537461727428293B0D0A7D0D0A0D0A0D0A69662820676C6F62616C2F546970362E476574416374696F6E53746172746564282920290D0A7B0D0A09676C6F62616C2F546970362E53746F7028293B0D0A7D0D0A0D0A2F2F204D6574726963730D0A676C6F62616C2F4D6574726963734D616E616765722F554957696E646F772E52657365744D657472696328293B0D0A676C6F62616C2F4D6574726963734D616E616765722F554957696E646F772E5365744D657472696344796E616D696356616C7565282257696E646F77222C20746869732E4765744E616D652829293B0D0A676C6F62616C2F4D6574726963734D616E616765722F554957696E646F772E5365744D657472696344796E616D696356616C75652822456C656D656E7456616C7565222C2022427574746F6E436C69636B22293B0D0A676C6F62616C2F4D6574726963734D616E616765722F554957696E646F772E53656E644D657472696328293B";

/// Called when the player presses on the quests button in the UI.
pub const GAME_QUESTS_CLICK: &str = "2F2F676C6F62616C2F536F756E644D697865722E536F756E644D69786572506C617928224755495F436C69636B31222C362C30293B0D0A0D0A69662820676C6F62616C2F517565737457696E646F772E476574416374696F6E53746172746564282920290D0A7B0D0A09676C6F62616C2F517565737457696E646F772E53746F7028293B0D0A7D0D0A656C73650D0A7B0D0A09676C6F62616C2F517565737457696E646F772E537461727428293B0D0A7D0D0A0D0A2F2F204D6574726963730D0A676C6F62616C2F4D6574726963734D616E616765722F554957696E646F772E52657365744D657472696328293B0D0A676C6F62616C2F4D6574726963734D616E616765722F554957696E646F772E5365744D657472696344796E616D696356616C7565282257696E646F77222C20746869732E4765744E616D652829293B0D0A676C6F62616C2F4D6574726963734D616E616765722F554957696E646F772E5365744D657472696344796E616D696356616C75652822456C656D656E7456616C7565222C2022427574746F6E436C69636B22293B0D0A676C6F62616C2F4D6574726963734D616E616765722F554957696E646F772E53656E644D657472696328293B";

/// Called when the player presses on the collections button in the UI.
pub const GAME_COLLECTIONS_CLICK: &str = "2F2F676C6F62616C2F536F756E644D697865722E536F756E644D69786572506C617928224755495F436C69636B31222C362C30293B0D0A0D0A676C6F62616C2F436F6C6C656374696F6E5669657747726F75702E546F67676C655669657747726F757028293B0D0A0D0A2F2F204D6574726963730D0A676C6F62616C2F4D6574726963734D616E616765722F554957696E646F772E52657365744D657472696328293B0D0A676C6F62616C2F4D6574726963734D616E616765722F554957696E646F772E5365744D657472696344796E616D696356616C7565282257696E646F77222C20746869732E4765744E616D652829293B0D0A676C6F62616C2F4D6574726963734D616E616765722F554957696E646F772E5365744D657472696344796E616D696356616C75652822456C656D656E7456616C7565222C2022427574746F6E436C69636B22293B0D0A676C6F62616C2F4D6574726963734D616E616765722F554957696E646F772E53656E644D657472696328293B";

/// Called when the player presses on the character sheet button in the UI:
pub const GAME_CSHEET_CLICK: &str = "2F2F676C6F62616C2F536F756E644D697865722E536F756E644D69786572506C617928224755495F436C69636B31222C362C30293B0D0A0D0A69662820676C6F62616C2F43686172616374657253686565742E476574416374696F6E53746172746564282920290D0A7B0D0A09676C6F62616C2F43686172616374657253686565742E53746F7028293B0D0A09676C6F62616C2F486F72736553686565742E53746F7028293B0D0A7D0D0A656C73650D0A7B0D0A09676C6F62616C2F43686172616374657253686565742E537461727428293B0D0A09676C6F62616C2F486F7273652E486F727365496E7370656374486F72736528293B0D0A7D0D0A0D0A0D0A0D0A69662820676C6F62616C2F546970352E476574416374696F6E53746172746564282920290D0A7B0D0A09676C6F62616C2F546970352E53746F7028293B0D0A7D0D0A0D0A2F2F204D6574726963730D0A676C6F62616C2F4D6574726963734D616E616765722F554957696E646F772E52657365744D657472696328293B0D0A676C6F62616C2F4D6574726963734D616E616765722F554957696E646F772E5365744D657472696344796E616D696356616C7565282257696E646F77222C20746869732E4765744E616D652829293B0D0A676C6F62616C2F4D6574726963734D616E616765722F554957696E646F772E5365744D657472696344796E616D696356616C75652822456C656D656E7456616C7565222C2022427574746F6E436C69636B22293B0D0A676C6F62616C2F4D6574726963734D616E616765722F554957696E646F772E53656E644D657472696328293B";

/// Introduction1 OnActionStart
pub const GAME_INTRO_START: &str = "2F2F202D2D2D2D2D2D2D2D2D2D2D2D2D2D2D2D2D2D2D2D2D2D2D2D2D2D2D2D2D2D2D2D2D2D2D2D2D2D2D2D2D2D2D2D2D2D2D2D2D2D2D2D2D2D2D2D2D2D2D2D2D2D2D2D2D2D2D2D2D2D0D0A2F2F202D2D2D2D2D2D2D2D2D2D2D2D2D2D2D2D2D2D2D2D2D2D2D2D2D2D2D2D2D2D2D2D2D2D2D2D2D2D2D2D2D2D2D2D2D2D2D2D2D2D2D2D2D2D2D2D2D2D2D2D2D2D2D2D2D2D2D2D2D2D0D0A2F2F2041422054657374202D20536B697020496E74726F204D6F7669652041202D20446F6E27742073686F7720496E74726F204D6F7669650D0A2F2F202D2D2D2D2D2D2D2D2D2D2D2D2D2D2D2D2D2D2D2D2D2D2D2D2D2D2D2D2D2D2D2D2D2D2D2D2D2D2D2D2D2D2D2D2D2D2D2D2D2D2D2D2D2D2D2D2D2D2D2D2D2D2D2D2D2D2D2D2D2D0D0A2F2F202D2D2D2D2D2D2D2D2D2D2D2D2D2D2D2D2D2D2D2D2D2D2D2D2D2D2D2D2D2D2D2D2D2D2D2D2D2D2D2D2D2D2D2D2D2D2D2D2D2D2D2D2D2D2D2D2D2D2D2D2D2D2D2D2D2D2D2D2D2D0D0A0D0A2F2F20436865636B2052656C65617365205461670D0A6966202820676C6F62616C2F526563757272696E674576656E744D616E616765722E476574526563757272696E674576656E744D616E616765725461674973416374697665282241425F536B6970496E74726F4D6F766965222920213D203120290D0A7B0D0A0972657475726E3B0D0A7D0D0A0D0A0D0A2F2F20436865636B20557365722049442028302C312C322C33292054686573652077696C6C20676574207468652063757272656E7420696E74726F64756374696F6E0D0A69662028207379732E476574546573745365676D656E744368617261637465722829203C3D203320290D0A7B0D0A0972657475726E3B0D0A7D0D0A0D0A2F2F20557365727320372D392C20706C61636520706C6179657273206469726563746C7920696E2067616D650D0A656C73652069662028207379732E476574546573745365676D656E744368617261637465722829203C3D203620290D0A7B0D0A676C6F62616C2F496E74726F64756374696F6E312E53746F7028293B0D0A7D0D0A0D0A2F2F73686F77206E65772063696E656D617469630D0A2F2F656C7365207B0D0A2F2F676C6F62616C2F496E74726F64756374696F6E312E53746F7028293B0D0A2F2F676C6F62616C2F41425F496E74726F566964656F2F47616D65496E74726F2F4375747363656E652E537461727428293B0D0A2F2F7D0D0A000800000053657276657273006F1900002F2F204465760D0A69662028207379732E476574546573745365676D656E745365727665722829203D3D202267732D77696E2D6465762E696E7465726E616C2E73746172737461626C652E636F6D2220297B20746869732F446174612E53657444617461496E742831293B7D0D0A0D0A0D0A2F2F2053746167696E670D0A69662028207379732E476574546573745365676D656E745365727665722829203D3D20223137322E31392E3235352E3137382220297B20746869732F446174612E53657444617461496E742831293B7D202F2F20514120310D0A2F2F2069662028207379732E476574546573745365676D656E745365727665722829203D3D20223137322E31392E3235342E3231352220297B20746869732F446174612E53657444617461496E742831293B7D202F2F20514120320D0A2F2F2069662028207379732E476574546573745365676D656E745365727665722829203D3D20223137322E31392E3235322E3232372220297B20746869732F446174612E53657444617461496E742831293B7D202F2F20514120330D0A2F2F2069662028207379732E476574546573745365676D656E745365727665722829203D3D20223137322E31392E3235352E3230342220297B20746869732F446174612E53657444617461496E742831293B7D202F2F20514120340D0A2F2F2069662028207379732E476574546573745365676D656E745365727665722829203D3D20223137322E31392E3235342E3134322220297B20746869732F446174612E53657444617461496E742831293B7D202F2F20514120350D0A2F2F2069662028207379732E476574546573745365676D656E745365727665722829203D3D20223137322E31392E3234342E3134382220297B20746869732F446174612E53657444617461496E742831293B7D202F2F20514120360D0A2F2F2069662028207379732E476574546573745365676D656E745365727665722829203D3D20223137322E31392E3234362E36352220297B20746869732F446174612E53657444617461496E742831293B7D202F2F20514120370D0A2F2F2069662028207379732E476574546573745365676D656E745365727665722829203D3D20223137322E31392E3234352E3139332220297B20746869732F446174612E53657444617461496E742831293B7D202F2F20514120380D0A2F2F2069662028207379732E476574546573745365676D656E745365727665722829203D3D20223137322E31392E3234322E36302220297B20746869732F446174612E53657444617461496E742831293B7D202F2F20514120390D0A0D0A0D0A2F2F204C6976650D0A0D0A2F2F204555524F50450D0A0D0A2F2F2044450D0A2F2F2069662028207379732E476574546573745365676D656E745365727665722829203D3D202235322E32382E35392E3231352220297B20746869732F446174612E53657444617461496E742831293B7D202F2F2073736F2D65752D64652D30310D0A2F2F2069662028207379732E476574546573745365676D656E745365727665722829203D3D202235322E32382E3139312E31362220297B20746869732F446174612E53657444617461496E742831293B7D202F2F2073736F2D65752D64652D30320D0A2F2F2069662028207379732E476574546573745365676D656E745365727665722829203D3D202235322E32382E3139312E3231312220297B20746869732F446174612E53657444617461496E742831293B7D202F2F2073736F2D65752D64652D30330D0A2F2F2069662028207379732E476574546573745365676D656E745365727665722829203D3D202235322E32392E3235342E33342220297B20746869732F446174612E53657444617461496E742831293B7D202F2F2073736F2D65752D64652D30340D0A2F2F2069662028207379732E476574546573745365676D656E745365727665722829203D3D202233352E3135362E3138312E3130302220297B20746869732F446174612E53657444617461496E742831293B7D202F2F2073736F2D65752D64652D30350D0A2F2F2069662028207379732E476574546573745365676D656E745365727665722829203D3D202233352E3135362E3139382E3230382220297B20746869732F446174612E53657444617461496E742831293B7D202F2F2073736F2D65752D64652D30360D0A2F2F2069662028207379732E476574546573745365676D656E745365727665722829203D3D202233352E3135372E38392E3234332220297B20746869732F446174612E53657444617461496E742831293B7D202F2F2073736F2D65752D64652D30370D0A2F2F2069662028207379732E476574546573745365676D656E745365727665722829203D3D202233352E3135362E32362E3135302220297B20746869732F446174612E53657444617461496E742831293B7D202F2F2073736F2D65752D64652D30380D0A2F2F2069662028207379732E476574546573745365676D656E745365727665722829203D3D202231382E3139352E3230312E34362220297B20746869732F446174612E53657444617461496E742831293B7D202F2F2073736F2D65752D64652D30390D0A0D0A2F2F2045450D0A2F2F2069662028207379732E476574546573745365676D656E745365727665722829203D3D202235322E32382E3139332E3231392220297B20746869732F446174612E53657444617461496E742831293B7D202F2F2073736F2D65752D65652D30310D0A2F2F2069662028207379732E476574546573745365676D656E745365727665722829203D3D202235322E32382E3139322E36322220297B20746869732F446174612E53657444617461496E742831293B7D202F2F2073736F2D65752D65652D30320D0A2F2F2069662028207379732E476574546573745365676D656E745365727665722829203D3D202235322E32382E3130362E302220297B20746869732F446174612E53657444617461496E742831293B7D202F2F2073736F2D65752D65652D30330D0A0D0A2F2F2045530D0A2F2F2069662028207379732E476574546573745365676D656E745365727665722829203D3D202235322E32382E34332E3132382220297B20746869732F446174612E53657444617461496E742831293B7D202F2F2073736F2D65752D65732D30310D0A0D0A2F2F2046490D0A2F2F2069662028207379732E476574546573745365676D656E745365727665722829203D3D202233352E3135382E312E31362220297B20746869732F446174612E53657444617461496E742831293B7D202F2F2073736F2D65752D66692D30310D0A0D0A2F2F2046520D0A2F2F2069662028207379732E476574546573745365676D656E745365727665722829203D3D202235322E32382E39372E32372220297B20746869732F446174612E53657444617461496E742831293B7D202F2F2073736F2D65752D66722D30310D0A2F2F2069662028207379732E476574546573745365676D656E745365727665722829203D3D202235322E32382E3135392E3132332220297B20746869732F446174612E53657444617461496E742831293B7D202F2F2073736F2D65752D66722D30320D0A2F2F2069662028207379732E476574546573745365676D656E745365727665722829203D3D202235322E35372E32312E3233322220297B20746869732F446174612E53657444617461496E742831293B7D202F2F2073736F2D65752D66722D30330D0A0D0A2F2F2047520D0A2F2F2069662028207379732E476574546573745365676D656E745365727665722829203D3D202235322E32382E3235332E38312220297B20746869732F446174612E53657444617461496E742831293B7D202F2F2073736F2D65752D67722D30310D0A0D0A2F2F2048550D0A69662028207379732E476574546573745365676D656E745365727665722829203D3D202235322E32382E3138342E32342220297B20746869732F446174612E53657444617461496E742831293B7D202F2F2073736F2D65752D68752D30310D0A2F2F2069662028207379732E476574546573745365676D656E745365727665722829203D3D202233352E3135362E38342E3139392220297B20746869732F446174612E53657444617461496E742831293B7D202F2F2073736F2D65752D68752D30320D0A2F2F2069662028207379732E476574546573745365676D656E745365727665722829203D3D202231382E3139342E3231342E32392220297B20746869732F446174612E53657444617461496E742831293B7D202F2F2073736F2D65752D68752D30330D0A0D0A2F2F2049540D0A2F2F2069662028207379732E476574546573745365676D656E745365727665722829203D3D202235322E32382E3139332E3134312220297B20746869732F446174612E53657444617461496E742831293B7D202F2F2073736F2D65752D69742D30310D0A0D0A2F2F204E4C0D0A2F2F2069662028207379732E476574546573745365676D656E745365727665722829203D3D202235322E32382E3138362E3232322220297B20746869732F446174612E53657444617461496E742831293B7D202F2F2073736F2D65752D6E6C2D30310D0A2F2F2069662028207379732E476574546573745365676D656E745365727665722829203D3D202235322E32382E3134382E3233332220297B20746869732F446174612E53657444617461496E742831293B7D202F2F2073736F2D65752D6E6C2D30320D0A2F2F2069662028207379732E476574546573745365676D656E745365727665722829203D3D202233352E3135362E3131392E3130302220297B20746869732F446174612E53657444617461496E742831293B7D202F2F2073736F2D65752D6E6C2D30330D0A2F2F2069662028207379732E476574546573745365676D656E745365727665722829203D3D202233352E3135362E35392E3235352220297B20746869732F446174612E53657444617461496E742831293B7D202F2F2073736F2D65752D6E6C2D30340D0A0D0A2F2F204E4F0D0A2F2F2069662028207379732E476574546573745365676D656E745365727665722829203D3D202233352E3135362E3137312E3131342220297B20746869732F446174612E53657444617461496E742831293B7D202F2F2073736F2D65752D6E6F2D30310D0A2F2F2069662028207379732E476574546573745365676D656E745365727665722829203D3D202235322E32382E3132352E36322220297B20746869732F446174612E53657444617461496E742831293B7D202F2F2073736F2D65752D6E6F2D30320D0A0D0A2F2F20504C0D0A2F2F2069662028207379732E476574546573745365676D656E745365727665722829203D3D202235322E32382E3138362E3231382220297B20746869732F446174612E53657444617461496E742831293B7D202F2F2073736F2D65752D706C2D30310D0A2F2F2069662028207379732E476574546573745365676D656E745365727665722829203D3D202235322E32382E31362E3233372220297B20746869732F446174612E53657444617461496E742831293B7D202F2F2073736F2D65752D706C2D30320D0A2F2F2069662028207379732E476574546573745365676D656E745365727665722829203D3D202235322E32382E3138302E36332220297B20746869732F446174612E53657444617461496E742831293B7D202F2F2073736F2D65752D706C2D30330D0A2F2F2069662028207379732E476574546573745365676D656E745365727665722829203D3D202233352E3135362E3134302E3132342220297B20746869732F446174612E53657444617461496E742831293B7D202F2F2073736F2D65752D706C2D30340D0A2F2F2069662028207379732E476574546573745365676D656E745365727665722829203D3D202233352E3135362E3137392E3131352220297B20746869732F446174612E53657444617461496E742831293B7D202F2F2073736F2D65752D706C2D30350D0A2F2F2069662028207379732E476574546573745365676D656E745365727665722829203D3D202233352E3135372E3136302E3138302220297B20746869732F446174612E53657444617461496E742831293B7D202F2F2073736F2D65752D706C2D30360D0A2F2F2069662028207379732E476574546573745365676D656E745365727665722829203D3D202231382E3139352E3136332E3135342220297B20746869732F446174612E53657444617461496E742831293B7D202F2F2073736F2D65752D706C2D30370D0A2F2F2069662028207379732E476574546573745365676D656E745365727665722829203D3D202231382E3139352E3231372E35372220297B20746869732F446174612E53657444617461496E742831293B7D202F2F2073736F2D65752D706C2D30380D0A0D0A2F2F2052550D0A2F2F2069662028207379732E476574546573745365676D656E745365727665722829203D3D202235322E32382E3139312E3230352220297B20746869732F446174612E53657444617461496E742831293B7D202F2F2073736F2D65752D72752D30310D0A2F2F2069662028207379732E476574546573745365676D656E745365727665722829203D3D202231382E3139352E3138302E3132362220297B20746869732F446174612E53657444617461496E742831293B7D202F2F2073736F2D65752D72752D30320D0A0D0A2F2F2053450D0A2F2F2069662028207379732E476574546573745365676D656E745365727665722829203D3D202235322E32382E3137372E33392220297B20746869732F446174612E53657444617461496E742831293B7D202F2F2073736F2D65752D73652D30310D0A2F2F2069662028207379732E476574546573745365676D656E745365727665722829203D3D202235322E32382E33322E34312220297B20746869732F446174612E53657444617461496E742831293B7D202F2F2073736F2D65752D73652D30320D0A2F2F2069662028207379732E476574546573745365676D656E745365727665722829203D3D202235322E32382E3138392E31372220297B20746869732F446174612E53657444617461496E742831293B7D202F2F2073736F2D65752D73652D30330D0A2F2F2069662028207379732E476574546573745365676D656E745365727665722829203D3D202233352E3135362E38362E3231352220297B20746869732F446174612E53657444617461496E742831293B7D202F2F2073736F2D65752D73652D30340D0A2F2F2069662028207379732E476574546573745365676D656E745365727665722829203D3D202233352E3135362E3139352E3231362220297B20746869732F446174612E53657444617461496E742831293B7D202F2F2073736F2D65752D73652D30350D0A2F2F2069662028207379732E476574546573745365676D656E745365727665722829203D3D202231382E3139352E3138352E3134352220297B20746869732F446174612E53657444617461496E742831293B7D202F2F2073736F2D65752D73652D30360D0A2F2F2069662028207379732E476574546573745365676D656E745365727665722829203D3D202233352E3135372E3139372E3232342220297B20746869732F446174612E53657444617461496E742831293B7D202F2F2073736F2D65752D73652D30370D0A0D0A2F2F20554B0D0A2F2F2069662028207379732E476574546573745365676D656E745365727665722829203D3D202235322E32382E3138332E3131342220297B20746869732F446174612E53657444617461496E742831293B7D202F2F2073736F2D65752D756B2D30310D0A2F2F2069662028207379732E476574546573745365676D656E745365727665722829203D3D202235342E39332E39362E3230392220297B20746869732F446174612E53657444617461496E742831293B7D202F2F2073736F2D65752D756B2D30320D0A2F2F2069662028207379732E476574546573745365676D656E745365727665722829203D3D202233352E3135362E3136302E3135332220297B20746869732F446174612E53657444617461496E742831293B7D202F2F2073736F2D65752D756B2D30330D0A0D0A2F2F2058580D0A2F2F2069662028207379732E476574546573745365676D656E745365727665722829203D3D202235322E32382E3138392E3132312220297B20746869732F446174612E53657444617461496E742831293B7D202F2F2073736F2D65752D78782D30310D0A2F2F2069662028207379732E476574546573745365676D656E745365727665722829203D3D202235322E32382E3137372E3137352220297B20746869732F446174612E53657444617461496E742831293B7D202F2F2073736F2D65752D78782D30320D0A2F2F2069662028207379732E476574546573745365676D656E745365727665722829203D3D202231382E3139352E37332E3230332220297B20746869732F446174612E53657444617461496E742831293B7D202F2F2073736F2D65752D78782D30330D0A0D0A0D0A2F2F204D4944444C4520454153540D0A0D0A2F2F2041450D0A2F2F2069662028207379732E476574546573745365676D656E745365727665722829203D3D202235322E32382E3135362E3137362220297B20746869732F446174612E53657444617461496E742831293B7D202F2F2073736F2D6D652D61652D30310D0A0D0A2F2F2054520D0A2F2F2069662028207379732E476574546573745365676D656E745365727665722829203D3D202235322E32382E3235332E37312220297B20746869732F446174612E53657444617461496E742831293B7D202F2F2073736F2D6D652D74722D30310D0A0D0A0D0A2F2F204E4F52544820414D45524943410D0A0D0A0D0A2F2F204F4345414E49410D0A0D0A0D0A2F2F20534F55544820414D4552494341";