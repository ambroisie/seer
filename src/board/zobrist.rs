// vim: foldmethod=marker
use super::{CastleRights, Color, File, Piece, Square};

type Hash = u64;

type EnPassantHashesType = [Hash; File::NUM_VARIANTS];

type CastlingHashesType = [[Hash; CastleRights::NUM_VARIANTS]; CastleRights::NUM_VARIANTS];

type PieceHashesType = [[[Hash; Square::NUM_VARIANTS]; Piece::NUM_VARIANTS]; Color::NUM_VARIANTS];

// region:sourcegen {{{
static BLACK_TO_MOVE: Hash = 64934999470316615;

static EN_PASSANT_HASHES: EnPassantHashesType = [
    15459456780870779090,
    13715484424881807779,
    17718572936700675021,
    14587996314750246637,
    6677028061025754322,
    13431580652429297681,
    10925346160689749684,
    14880644562654141744,
];

static CASTLING_HASHES: CastlingHashesType = [
    [
        1045274958981515586,
        5392148996167941106,
        14319713487061152634,
        15981422700840256970,
    ],
    [
        11438431953559128973,
        15165640837126078406,
        9682374909294934964,
        3746638112738371362,
    ],
    [
        13000030342210934332,
        12451924122435870810,
        1313989399406774579,
        3743948554594595520,
    ],
    [
        1870378950492858291,
        9613620781770901799,
        1269273018318959782,
        13819231333457059,
    ],
];

static PIECE_HASHES: PieceHashesType = [
    [
        [
            15846310122511714997,
            15565321750356536561,
            3955109975630601799,
            3742215252323203816,
            11547754013648268969,
            9179750610831012912,
            10024580902115404844,
            13461155397890310677,
            3979252975486053221,
            6862564799914328471,
            15012810188093697577,
            14104378448227943386,
            2251111072992350966,
            8718770409981886807,
            11590981376022443418,
            9179472875549373335,
            14751486135220696139,
            14770349609267618816,
            15794210789350854238,
            8074262070310026138,
            9807231349326167510,
            17878059168006197756,
            17020217056933277306,
            8168773538726665343,
            10540649568352071500,
            16427391340497996472,
            12836595115039890498,
            13099644563326246186,
            15638621634497942258,
            2470341953153926693,
            17228296995745787514,
            12051949271418111299,
            6735994487591330742,
            9945519289001331203,
            8761273995014635795,
            1134254747965098098,
            149908616240508937,
            947996201587548444,
            10050792753492913065,
            6389723739863027484,
            4054114923002827406,
            12342721822805852545,
            5603619551769484817,
            1502519545082924996,
            6629729157787095604,
            6505804100374502042,
            6551044912839567696,
            17519353744604830596,
            18262046566844392781,
            8444802205465859643,
            12318372284896528480,
            16384512768777965260,
            13872119029325166208,
            551553250648480702,
            12194494366773487659,
            8903817316641024797,
            2856044415979638552,
            17626060259890197188,
            6204959673627423824,
            1196712911958154145,
            2299761267346255152,
            13890713223244418265,
            13545009731095633715,
            5588589786121212040,
        ],
        [
            13394976401688450359,
            13458262092976265526,
            6458425333551298230,
            16068454197931831293,
            18115196974811588644,
            1945822552233231513,
            18242988029158079236,
            3508136533350863061,
            9221855519679203686,
            3417028073401167173,
            13676868907893858811,
            7891128041711130824,
            4438008134626964309,
            828375571561672767,
            5211525107864059583,
            9951912569669194338,
            15797222372094286515,
            8031369593312222855,
            8459483884800911883,
            16250516697391580568,
            9104850039807961769,
            9638459339574322133,
            7406414063314534801,
            16587044842087257956,
            4325976508659744589,
            3356271001861605262,
            11946543884567451524,
            2812959899045266534,
            9351004795482833123,
            12853162644250497974,
            15431128269247189177,
            5991459945115953374,
            10350914579691276699,
            317376031773017443,
            3618864447183195700,
            16484606785691860878,
            9692191468962987334,
            37863413628624730,
            16923022336523863681,
            5326304323994646307,
            2428164027580304344,
            10876721990417417330,
            420071405568015327,
            15896189023987324028,
            7501421402327487769,
            14082520727391598306,
            11398443591517781683,
            11126740362224499141,
            6822215190080389784,
            14824879461080717187,
            10688928270409693617,
            13901361354457603851,
            250990532055096151,
            217795811433087721,
            11496704042595325298,
            12623825688587535239,
            6447866609161442272,
            11475096638962239574,
            13961623773416109785,
            4105307170473648810,
            3973243685325579613,
            9494401018484929627,
            20006943945916687,
            11903697614114951560,
        ],
        [
            3690587531763593363,
            13885624810608316834,
            5607624542457669773,
            12135564358801855460,
            4446836951225686915,
            12472483031571860689,
            2736997872307808559,
            118674841525456732,
            17193145968198975121,
            7969033855049787458,
            5737495496259253001,
            1897399075984783518,
            12789381638949760189,
            8113504179765797443,
            595432870435327398,
            3476852355417867968,
            8280596186023326035,
            7031315074821615754,
            15130713716303029382,
            14738515924140621785,
            16823656838416817865,
            11512745403171831959,
            9713600364975496703,
            8076259513948145927,
            14467669620925674481,
            158730839338002039,
            1802592438558145119,
            9363119952329021091,
            105967970713505430,
            15995787761535739532,
            1909056520202569570,
            3849657788135620047,
            1433706242814734002,
            2512105352812598672,
            2887501559413438146,
            15210005340970705953,
            10881040147030968516,
            7578876291932537631,
            12227032199200032249,
            5720045455042277852,
            17400393972124160940,
            13968348826696289700,
            4691947163184896321,
            14382862859392087454,
            1273370694834677574,
            2097899237423795441,
            15051632852246180430,
            16165677388051025154,
            2535027998345740898,
            6612021553031301778,
            4872500892936395322,
            9213127058723692479,
            16637311583034884253,
            17157822031284380900,
            1657922974346786987,
            11175106282670399340,
            15892468403830849290,
            4101255317824574327,
            14501119344146015188,
            3420256760684759966,
            430472134306562862,
            2337444395187388666,
            13599329138906238118,
            17668204307700984669,
        ],
        [
            1767472920679339057,
            14082936559468749559,
            1413880663326372116,
            10705367841426333649,
            5537470338015349051,
            12866924289831421291,
            4428751101812181690,
            2699958690904306254,
            18213591554206267200,
            8527893759735728323,
            7172880010449874732,
            12672600755971214662,
            12210003153898278039,
            8555607754266391982,
            14398998151357989081,
            858080552587592614,
            15504853585397258680,
            6220555144450628459,
            10305515006459536590,
            14891452017879003363,
            18050619291989201021,
            11373790071747978079,
            18201773829943487338,
            10437978645586997949,
            12849819919940976821,
            4483246683417136815,
            15846903660579554392,
            1018784135202123164,
            6911185105132183235,
            11619014052459059223,
            8818501332496366977,
            15546893450170930577,
            16950209698165104902,
            4116125396588831597,
            188473686990703848,
            18187207258885063840,
            16363818859628459401,
            8743990205247337841,
            10488087234691240397,
            5651938883350096610,
            15924019588565743041,
            11136159581173086291,
            17934822160030264765,
            5061298994379818785,
            195369924570726370,
            17050832301390611091,
            22725132194490493,
            15556414533079398843,
            3990573830772882378,
            3360169762087184725,
            689245566469322678,
            8931066757019732136,
            7719317277364154882,
            226304218713923770,
            4943374134151180893,
            13731300347285222865,
            301214310880715317,
            389377574110938037,
            14173618363053396331,
            3431181573289799396,
            13770322358880719908,
            14755876294387681197,
            1810194810765833667,
            10108745010013199269,
        ],
        [
            7764062839457162437,
            11741759478077697033,
            8651354243877205790,
            15429384648560453166,
            4282651702327747008,
            16871187978641026815,
            6820994512466779277,
            18158156566427459046,
            4508524102950979090,
            16396223406243106025,
            11240522534199142118,
            792740089807455455,
            15550355798864849911,
            11686348911904530676,
            6040771785522271236,
            13488759770116750509,
            4731594914694946817,
            1040231234387996200,
            8849499839857637945,
            2413055014148664122,
            13867290405531250176,
            14362190972884924003,
            17305062087483598891,
            1748847742159412754,
            8133811204974116306,
            12083718907601203988,
            7014018663613897084,
            2014092656610274856,
            2743298157667671447,
            11643732090855831635,
            11250851525006141623,
            2936428779704708301,
            11393067578885422743,
            8724015120353455274,
            13705068541601393894,
            7528131639007409723,
            15141613737485729718,
            3385537796990715435,
            6570183427959510278,
            12751050629918821094,
            7646275094027983157,
            8277005368415815292,
            7546636328154591228,
            15648771470751997165,
            13982129828433177322,
            10180436254617215165,
            392996715048618277,
            13232777498271787105,
            7055760385863776354,
            13254342896708451569,
            8560140581582040378,
            107109168603320886,
            18230858113827520737,
            5282414160039311498,
            7529636915621456382,
            15616347824638058603,
            11147296505326367251,
            542673254595523470,
            16954422334495305532,
            11364052937546887148,
            2888990270943122866,
            18091936069917916498,
            6281124045212188877,
            8593493261528980987,
        ],
        [
            2989769720598162601,
            9106605827001636129,
            9570364881960985889,
            12902387062392871952,
            6239306021096882609,
            193926176087719705,
            6039110239366139997,
            15976034333201272181,
            1284657161207483125,
            4529520247997302077,
            12880558114497078927,
            6431387664297095305,
            15752238407942406005,
            7882692850296881944,
            8729163884851061539,
            14047483969824405122,
            17478776223161311773,
            16826998636345075644,
            5858086577301950025,
            2636153573532075124,
            17699473427841256248,
            1799999873579346924,
            17566984640879707528,
            13988132095580126052,
            7619590186098772590,
            618624160438839140,
            12219561462441938361,
            7148930398504884567,
            12422992075996304722,
            13118697411912473096,
            13367073180995828371,
            10610114944822631745,
            12078360036519044653,
            17421137131317266064,
            1615990355538785141,
            10376900783894145643,
            7488144083843255003,
            7517365090928813948,
            321937459601209105,
            10902930400532188781,
            16291893235893524057,
            9427884403524285839,
            13578219403996237208,
            7806344414248555555,
            1590178323452864480,
            4502737751334025566,
            14708705959315399774,
            17590011802110929593,
            8011063244042503593,
            3372062355514862432,
            7956604703943654557,
            7231568622761101113,
            3263790014618620510,
            5386114617935898764,
            17264930726535012718,
            17080979445745211883,
            16389446497941026493,
            1251555922590127803,
            1766319409423742345,
            5824933421221698593,
            8025838020465500614,
            1712486214309232600,
            2515668779116428348,
            5226880572965474519,
        ],
    ],
    [
        [
            8784996223880854135,
            8383322499467679771,
            11940829602286487528,
            749107577493620337,
            11658422359938522809,
            4437497841247955656,
            15039671436672630417,
            4428360460268361808,
            2612794400702075079,
            15154659025882080833,
            5966000488771745326,
            4682695811674311589,
            12399365904562251200,
            5743563134163369275,
            13725569263199228994,
            7820192892738641522,
            6725918893915352363,
            7238103260086822739,
            2419261908419519043,
            1944371521922958171,
            2030558929543619751,
            3154111065943957722,
            4897988930870536672,
            1998798151709288163,
            8648130913002646752,
            14934102049464497415,
            14890708649339335861,
            5049764018185114710,
            6628788106932185325,
            7112428968720217947,
            768481401627884428,
            6927893648564764526,
            14824457410356736781,
            4919540991021201863,
            17657853678445794877,
            2455519753056386028,
            16172661961128836972,
            12167841863002121500,
            13316779136479539099,
            18284102055623331862,
            17589865849446474652,
            6717797920114441655,
            15524331207306797155,
            17070876216372501673,
            1954875790784141799,
            10645093897317790329,
            4708222068247433732,
            4356248055106363889,
            10404059695057428683,
            13035281567593499713,
            8349335668474539561,
            3990743337244382716,
            14952065725070842007,
            6950645437249564545,
            5652844207558347571,
            6281201628596327010,
            10838601833556911887,
            3078232460409103108,
            16757569558796272077,
            13517006479681764308,
            3967782207298453705,
            2714491513812003904,
            3317235162126665239,
            10226456545359395820,
        ],
        [
            14251164629646129355,
            14998010409278028449,
            15542716158726314377,
            15397369278605634912,
            6100939107550711871,
            8713557161650028897,
            7089200899188634784,
            18063407508254720661,
            4337632717401767199,
            412125615877358058,
            6028165109585501223,
            15363085808083465317,
            14613845430069397737,
            14144649511022558651,
            8723888308700229883,
            14293487351767918335,
            9715937204640118456,
            11580361794788080335,
            915367859885060415,
            6459021974772294395,
            12060251587777088927,
            5532704576266363546,
            14489993909470049835,
            16426177833267107097,
            3607428550576839172,
            7438257254616020826,
            1396656355772112991,
            8608813051183737928,
            16492817754651347629,
            17881274662026570031,
            2300492472172319558,
            4404464307798321562,
            673784220717200369,
            821776834022136343,
            9919609767573651109,
            683748932020981069,
            8525746577539403297,
            6457801651776444340,
            9646366338259658387,
            13892539345304574027,
            17083943815740411827,
            14429863601822528150,
            12168108271452973589,
            7112427582244629105,
            15323648062786793762,
            17973045586494826124,
            12929297255917642880,
            7541633487917495391,
            16210663046879785083,
            6910370472905777769,
            8343768604822167659,
            3485907538729594342,
            7103934160758964305,
            4901126205516980066,
            314770954154071230,
            13789286550919187294,
            17135937634126147737,
            17711186974010444553,
            3583060176114895621,
            12984377250840503307,
            9001292467502639703,
            5302703806177083024,
            11829855060898447362,
            7792871150005700771,
        ],
        [
            1520093331355966943,
            14776253866741286711,
            1830435902514789447,
            6783716807773264245,
            7030091404042309875,
            1720223441255340701,
            13445382365734258737,
            11449213796602164673,
            10874370555683857320,
            18246047722161781849,
            492037621672777879,
            16532473357347977151,
            5888002492723757072,
            11847570994151287255,
            14050715334936968126,
            5448278994699028448,
            5062909158550936849,
            11465405229960447949,
            2779995984806079730,
            8108449415663851271,
            5925771076410909364,
            4398525992925930953,
            15105632689435380115,
            8224881353639377989,
            13142137441375466514,
            402244963327403059,
            15972426265038828054,
            14918608839495061965,
            4783523652206804079,
            5533802637901392118,
            15857468142361536438,
            483935465300448184,
            17547570023467671419,
            9118083899124499413,
            16067662280423557207,
            14860481864823305290,
            12544461391667452601,
            16479998523741582369,
            2004653907916258288,
            10016134492966295503,
            14654098897665700345,
            15597902928417621210,
            17875771155388593350,
            12187680377949566458,
            6591854158971319321,
            10921345154959883977,
            6992586914959052076,
            6881186710214967907,
            10133687459128153147,
            11053978924568326706,
            13937927219834850327,
            3101883468469491331,
            8427212063981998124,
            2092323926080990908,
            3585817315360256386,
            15747726726131045899,
            16440560833072674588,
            18384346789732354577,
            16783457322743335580,
            11756627869063466658,
            11167824433661290940,
            9877633665654560039,
            17040076472624122335,
            3220010953426300691,
        ],
        [
            2440551681152908217,
            17542921143062722953,
            11172685451694101778,
            1048396501496535768,
            6426140309273374561,
            12612852760901936331,
            1512574622848540746,
            12173187909250955505,
            2022797643158268462,
            11997025710748266437,
            7697722427032893513,
            3131801314231591172,
            16260489484426716331,
            4160620514044135000,
            2020718294955480764,
            3402776113237843614,
            1763635255903222910,
            1493348699626208256,
            11968167413422708151,
            345811979331432929,
            6373077428271171816,
            2460378116234706383,
            7628144846272841726,
            13325719783294577659,
            14728551694622182478,
            3801351080956315294,
            14893478564914456114,
            6842729530296982472,
            7092303763023731054,
            13253731529913205555,
            12120103223599095759,
            13535231015523935415,
            10292012323401205180,
            1773004715998349698,
            16799776551041782060,
            10929981434226220413,
            5672227401526069291,
            15197185234723998232,
            2327868618089768447,
            11691196257790359528,
            7797108122232849111,
            17247378053385545795,
            17190992283359664342,
            11765180081852900764,
            17136020481758062916,
            16631142675265592717,
            2780308612474172011,
            13623309349488327240,
            10027477427374497277,
            10901098973779551879,
            13622367348979020321,
            17240066494945993895,
            3613111118313098801,
            3595718136710251489,
            15541177346768674456,
            2745954478213783820,
            2993333600340807291,
            7964104343962539977,
            2018911365155885783,
            303730231039108799,
            8347454704269268605,
            15168982392769178779,
            2222543894286897630,
            7116612461922271036,
        ],
        [
            6659924125020249146,
            6447320752673406213,
            9967928188577790738,
            13067246238653248617,
            17543091787100809270,
            8900021509039330195,
            17145380139140011656,
            8400586664049469607,
            10795739003701585879,
            15466110299051417899,
            12359481046676402302,
            7116524366403677718,
            16169260265445853433,
            8692955756594060325,
            16615201418588912179,
            13173840077278718451,
            11541289024103137321,
            6319485103686695464,
            17758367064472002749,
            13554787615980332253,
            11638126900518821099,
            13954358659690479765,
            9973591566221607183,
            17163362073809978216,
            18303884563040881112,
            17405285323670017057,
            342959662696107471,
            1347610445893916230,
            5966782602002155351,
            7019395552225934922,
            5023307274424494080,
            14877119153064003138,
            12713062183167165488,
            3300175320814166235,
            12719695066185815582,
            15494003269637617920,
            12471806264411606594,
            689503650964098331,
            13045361994561212689,
            6792032840206349333,
            8564825371269942766,
            11860222023510411541,
            8263360838969018477,
            4621300043926690155,
            4911502441925492974,
            5810422953738103400,
            11263635798182361717,
            9404485481139860053,
            15617136572270427657,
            2446567841402084273,
            8338408414503449551,
            15353996633614181426,
            4523550137711543282,
            12457522381881075539,
            4092390258313244045,
            3708028013704908406,
            16685460470125872576,
            16440216368869048512,
            12879711695951990697,
            4790961591497355085,
            8914134962745961064,
            15580015213313907777,
            8644499442940218319,
            61217168966658081,
        ],
        [
            2912759343743843999,
            14884813698979495195,
            16425233319897104847,
            14688503175962192854,
            16633510544151871827,
            712664039270274475,
            585745436804207639,
            7392141637871392475,
            18381405436219133756,
            14689295466480430970,
            9306876723096610361,
            12658459328140256364,
            3007342023158025032,
            4037157658642530533,
            8009151754207580377,
            4847981282259436185,
            1433660584766270169,
            5274091252087296014,
            13330399601366117612,
            16588395809568307079,
            14448566552430231758,
            13343329601558313195,
            12158146930071854376,
            4197885034159347546,
            3445100139072421635,
            16609318396120447823,
            6042075030399342756,
            13793584353850054662,
            16021579771298913846,
            11054052814289128888,
            16074104597928497109,
            15130554541552071802,
            4895805849687331062,
            1580544981451361794,
            2425277209291829232,
            7864782989239724936,
            17589104864303907651,
            15274489251065018408,
            5101250638773604988,
            2484123793698223520,
            5015053599434799784,
            16436209554247109768,
            10530941408166861910,
            7952403580920522728,
            14690149799888783296,
            3789888246444727158,
            18109247358095710450,
            3088677810223885810,
            5022612625499977309,
            7112612464628965285,
            4566226729227242637,
            5042221516346092028,
            16846599512836514536,
            6256199833536459738,
            2014937527629117588,
            17739428229013858448,
            13982298082608600032,
            17248258267345510543,
            18214829730500207223,
            7600462139109877437,
            6812119576048580698,
            6424206435528040580,
            487857718885957481,
            13371986807426798615,
        ],
    ],
];
// endregion:sourcegen }}}

/// Return the Zobrist hash for a [Piece] of a given [Color] on a given [Square].
pub fn moved_piece(color: Color, piece: Piece, square: Square) -> Hash {
    PIECE_HASHES[color.index()][piece.index()][square.index()]
}

/// Return the Zobrist hash for the [CastleRights] for a [Color].
pub fn castling_rights(rights: [CastleRights; Color::NUM_VARIANTS]) -> Hash {
    CASTLING_HASHES[rights[0].index()][rights[1].index()]
}

/// Return the Zobrist hash for the [File] of an en-passant capture.
pub fn en_passant(file: File) -> Hash {
    EN_PASSANT_HASHES[file.index()]
}

/// Return the Zobrist hash for the given side-to-move being [Color::Black].
pub fn side_to_move() -> Hash {
    BLACK_TO_MOVE
}

#[cfg(test)]
mod test {
    use std::fmt::{Display, Write as _};

    use super::*;
    use crate::utils::SimpleRng;

    fn split_twice<'a>(
        text: &'a str,
        start_marker: &str,
        end_marker: &str,
    ) -> Option<(&'a str, &'a str, &'a str)> {
        let (prefix, rest) = text.split_once(start_marker)?;
        let (mid, suffix) = rest.split_once(end_marker)?;
        Some((prefix, mid, suffix))
    }

    fn array_string<T: Display>(indent_level: usize, values: &[T]) -> String {
        let inner = || -> Result<String, std::fmt::Error> {
            let mut res = String::new();

            writeln!(&mut res, "[")?;
            for val in values {
                let indent = (indent_level + 1) * 4;
                writeln!(&mut res, "{:indent$}{},", "", val)?;
            }
            let indent = indent_level * 4;
            write!(&mut res, "{:indent$}]", "")?;

            Ok(res)
        };

        inner().unwrap()
    }

    #[test]
    fn regen_zobrist_hashes() {
        let mut rng = SimpleRng::new();

        macro_rules! rng_iter {
            ($outer:expr, $( $inner:expr ),+) => {{
                let mut res = vec![];
                for _ in $outer {
                    res.push(rng_iter!($($inner),+));
                }
                res
            }};
            ($iter:expr) => {{
                let mut res = vec![];
                for _ in $iter {
                    res.push(rng.gen());
                }
                res
            }};
        }

        let black_to_move = rng.gen();
        let en_passant = rng_iter!(File::iter());
        let castle_rights = rng_iter!(CastleRights::iter(), CastleRights::iter());
        let move_piece = rng_iter!(Color::iter(), Piece::iter(), Square::iter());

        let original_text = std::fs::read_to_string(file!()).unwrap();
        let new_text = {
            let start_marker = "// region:sourcegen {{{\n";
            let end_marker = "// endregion:sourcegen }}}\n";
            let (prefix, _, suffix) =
                split_twice(&original_text, start_marker, end_marker).unwrap();

            let black_to_move = format!("static BLACK_TO_MOVE: Hash = {};\n", black_to_move);
            let en_passant = format!(
                "static EN_PASSANT_HASHES: EnPassantHashesType = {};\n",
                array_string(0, &en_passant[..]),
            );
            let castle_rights = {
                let strings: Vec<_> = castle_rights
                    .iter()
                    .map(|v| array_string(1, &v[..]))
                    .collect();
                format!(
                    "static CASTLING_HASHES: CastlingHashesType = {};\n",
                    array_string(0, &strings[..])
                )
            };
            let move_piece = {
                let strings: Vec<_> = move_piece
                    .iter()
                    .map(|v| {
                        let strings: Vec<_> = v.iter().map(|v| array_string(2, &v[..])).collect();
                        array_string(1, &strings[..])
                    })
                    .collect();
                format!(
                    "static PIECE_HASHES: PieceHashesType = {};\n",
                    array_string(0, &strings[..])
                )
            };
            let hashes = format!("{black_to_move}\n{en_passant}\n{castle_rights}\n{move_piece}");
            format!("{prefix}{start_marker}{hashes}{end_marker}{suffix}")
        };

        if new_text != original_text {
            std::fs::write(file!(), new_text).unwrap();
            panic!("source was not up-to-date")
        }
    }
}
