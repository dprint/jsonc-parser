window.BENCHMARK_DATA = {
  "lastUpdate": 1764212748060,
  "repoUrl": "https://github.com/dprint/jsonc-parser",
  "entries": {
    "Benchmark": [
      {
        "commit": {
          "author": {
            "email": "dsherret@gmail.com",
            "name": "David Sherret",
            "username": "dsherret"
          },
          "committer": {
            "email": "dsherret@gmail.com",
            "name": "David Sherret",
            "username": "dsherret"
          },
          "distinct": true,
          "id": "f79d24cc3ef3c1ce2517c4db7ff654b9a7cc02fb",
          "message": "chore: Fix benchmark deploy step.",
          "timestamp": "2021-04-02T16:40:29-04:00",
          "tree_id": "d10dced634b4e4e6768ce11ca4f205086354b4fd",
          "url": "https://github.com/dprint/jsonc-parser/commit/f79d24cc3ef3c1ce2517c4db7ff654b9a7cc02fb"
        },
        "date": 1617396172463,
        "tool": "cargo",
        "benches": [
          {
            "name": "citm_catalog_json_large_ast",
            "value": 140988567,
            "range": "± 5677583",
            "unit": "ns/iter"
          },
          {
            "name": "citm_catalog_json_large_value",
            "value": 165642972,
            "range": "± 3980526",
            "unit": "ns/iter"
          },
          {
            "name": "package_json_ast",
            "value": 172275,
            "range": "± 33391",
            "unit": "ns/iter"
          },
          {
            "name": "package_json_value",
            "value": 214306,
            "range": "± 54631",
            "unit": "ns/iter"
          },
          {
            "name": "tsconfig_json_ast",
            "value": 50563,
            "range": "± 10570",
            "unit": "ns/iter"
          },
          {
            "name": "tsconfig_json_value",
            "value": 50986,
            "range": "± 5492",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "dsherret@gmail.com",
            "name": "David Sherret",
            "username": "dsherret"
          },
          "committer": {
            "email": "dsherret@gmail.com",
            "name": "David Sherret",
            "username": "dsherret"
          },
          "distinct": true,
          "id": "07bbde4620374dd37ddc8334162d833ca98e2e80",
          "message": "chore(scanner): Use a char iterator instead of allocating a vector of chars.",
          "timestamp": "2021-04-02T17:47:05-04:00",
          "tree_id": "539c25ee9488fbc87ef0978e81ac1fa4c60ad39d",
          "url": "https://github.com/dprint/jsonc-parser/commit/07bbde4620374dd37ddc8334162d833ca98e2e80"
        },
        "date": 1617400161835,
        "tool": "cargo",
        "benches": [
          {
            "name": "citm_catalog_json_large_ast",
            "value": 134697670,
            "range": "± 7009667",
            "unit": "ns/iter"
          },
          {
            "name": "citm_catalog_json_large_value",
            "value": 160727909,
            "range": "± 7575121",
            "unit": "ns/iter"
          },
          {
            "name": "package_json_ast",
            "value": 212512,
            "range": "± 9091",
            "unit": "ns/iter"
          },
          {
            "name": "package_json_value",
            "value": 255581,
            "range": "± 8492",
            "unit": "ns/iter"
          },
          {
            "name": "tsconfig_json_ast",
            "value": 70738,
            "range": "± 1414",
            "unit": "ns/iter"
          },
          {
            "name": "tsconfig_json_value",
            "value": 71150,
            "range": "± 3672",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "dsherret@gmail.com",
            "name": "David Sherret",
            "username": "dsherret"
          },
          "committer": {
            "email": "dsherret@gmail.com",
            "name": "David Sherret",
            "username": "dsherret"
          },
          "distinct": true,
          "id": "84f3b537355e866dc5389d822ed10c95c9df6b5f",
          "message": "perf: Comment text references original string (part of #10)",
          "timestamp": "2021-04-02T18:06:06-04:00",
          "tree_id": "1663de640af21a1a315843595158fd19dd31b57d",
          "url": "https://github.com/dprint/jsonc-parser/commit/84f3b537355e866dc5389d822ed10c95c9df6b5f"
        },
        "date": 1617401316667,
        "tool": "cargo",
        "benches": [
          {
            "name": "citm_catalog_json_large_ast",
            "value": 123925235,
            "range": "± 8943739",
            "unit": "ns/iter"
          },
          {
            "name": "citm_catalog_json_large_value",
            "value": 154665110,
            "range": "± 17871619",
            "unit": "ns/iter"
          },
          {
            "name": "package_json_ast",
            "value": 178589,
            "range": "± 19557",
            "unit": "ns/iter"
          },
          {
            "name": "package_json_value",
            "value": 216027,
            "range": "± 20409",
            "unit": "ns/iter"
          },
          {
            "name": "tsconfig_json_ast",
            "value": 43346,
            "range": "± 5490",
            "unit": "ns/iter"
          },
          {
            "name": "tsconfig_json_value",
            "value": 42185,
            "range": "± 6088",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "dsherret@gmail.com",
            "name": "David Sherret",
            "username": "dsherret"
          },
          "committer": {
            "email": "dsherret@gmail.com",
            "name": "David Sherret",
            "username": "dsherret"
          },
          "distinct": true,
          "id": "620807aa79d639cd69d6147afa2ab4a474c439a4",
          "message": "perf: Reference original string for word and number literals (More work for #10)",
          "timestamp": "2021-04-02T18:45:06-04:00",
          "tree_id": "d909147c722d2e11006507d403e96eec16d16821",
          "url": "https://github.com/dprint/jsonc-parser/commit/620807aa79d639cd69d6147afa2ab4a474c439a4"
        },
        "date": 1617403665573,
        "tool": "cargo",
        "benches": [
          {
            "name": "citm_catalog_json_large_ast",
            "value": 140116759,
            "range": "± 18228302",
            "unit": "ns/iter"
          },
          {
            "name": "citm_catalog_json_large_value",
            "value": 162037660,
            "range": "± 12535190",
            "unit": "ns/iter"
          },
          {
            "name": "package_json_ast",
            "value": 204224,
            "range": "± 39735",
            "unit": "ns/iter"
          },
          {
            "name": "package_json_value",
            "value": 242315,
            "range": "± 40806",
            "unit": "ns/iter"
          },
          {
            "name": "tsconfig_json_ast",
            "value": 48086,
            "range": "± 7287",
            "unit": "ns/iter"
          },
          {
            "name": "tsconfig_json_value",
            "value": 50842,
            "range": "± 7743",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "dsherret@gmail.com",
            "name": "David Sherret",
            "username": "dsherret"
          },
          "committer": {
            "email": "dsherret@gmail.com",
            "name": "David Sherret",
            "username": "dsherret"
          },
          "distinct": true,
          "id": "c2777030c13adf4162fefed691d8fefe127d8266",
          "message": "perf: #10 - Reference the original string for string literals more often.",
          "timestamp": "2021-04-02T19:26:21-04:00",
          "tree_id": "6cfb8f70bd28056c715fef8fdaa959786fd7f155",
          "url": "https://github.com/dprint/jsonc-parser/commit/c2777030c13adf4162fefed691d8fefe127d8266"
        },
        "date": 1617406128515,
        "tool": "cargo",
        "benches": [
          {
            "name": "citm_catalog_json_large_ast",
            "value": 124763267,
            "range": "± 6681993",
            "unit": "ns/iter"
          },
          {
            "name": "citm_catalog_json_large_value",
            "value": 161634298,
            "range": "± 13965490",
            "unit": "ns/iter"
          },
          {
            "name": "package_json_ast",
            "value": 135194,
            "range": "± 15942",
            "unit": "ns/iter"
          },
          {
            "name": "package_json_value",
            "value": 183321,
            "range": "± 30977",
            "unit": "ns/iter"
          },
          {
            "name": "tsconfig_json_ast",
            "value": 49489,
            "range": "± 6299",
            "unit": "ns/iter"
          },
          {
            "name": "tsconfig_json_value",
            "value": 50987,
            "range": "± 5197",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "dsherret@gmail.com",
            "name": "David Sherret",
            "username": "dsherret"
          },
          "committer": {
            "email": "dsherret@gmail.com",
            "name": "David Sherret",
            "username": "dsherret"
          },
          "distinct": true,
          "id": "0afe9434e7e5e99fed768761a5e9be7882c91a80",
          "message": "chore(release): 0.16.0",
          "timestamp": "2021-04-02T20:14:46-04:00",
          "tree_id": "9d51a07b06c77b8fc5d1050d193edeb1ad847745",
          "url": "https://github.com/dprint/jsonc-parser/commit/0afe9434e7e5e99fed768761a5e9be7882c91a80"
        },
        "date": 1617408984195,
        "tool": "cargo",
        "benches": [
          {
            "name": "citm_catalog_json_large_ast",
            "value": 88782896,
            "range": "± 329714",
            "unit": "ns/iter"
          },
          {
            "name": "citm_catalog_json_large_value",
            "value": 108709343,
            "range": "± 1196432",
            "unit": "ns/iter"
          },
          {
            "name": "package_json_ast",
            "value": 91363,
            "range": "± 896",
            "unit": "ns/iter"
          },
          {
            "name": "package_json_value",
            "value": 132250,
            "range": "± 1590",
            "unit": "ns/iter"
          },
          {
            "name": "tsconfig_json_ast",
            "value": 35477,
            "range": "± 180",
            "unit": "ns/iter"
          },
          {
            "name": "tsconfig_json_value",
            "value": 36191,
            "range": "± 168",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "dsherret@gmail.com",
            "name": "David Sherret",
            "username": "dsherret"
          },
          "committer": {
            "email": "dsherret@gmail.com",
            "name": "David Sherret",
            "username": "dsherret"
          },
          "distinct": true,
          "id": "a2b22834c658ba610cd40ed53e323acf0f8b83fd",
          "message": "feat: #5 - Add `parse_to_serde_value` under \"serde\" feature.",
          "timestamp": "2021-04-03T11:36:58-04:00",
          "tree_id": "732414acf1164cac417a57ce7972507a9a986700",
          "url": "https://github.com/dprint/jsonc-parser/commit/a2b22834c658ba610cd40ed53e323acf0f8b83fd"
        },
        "date": 1617464345445,
        "tool": "cargo",
        "benches": [
          {
            "name": "citm_catalog_json_large_ast",
            "value": 118338273,
            "range": "± 600406",
            "unit": "ns/iter"
          },
          {
            "name": "citm_catalog_json_large_value",
            "value": 141815266,
            "range": "± 1515681",
            "unit": "ns/iter"
          },
          {
            "name": "package_json_ast",
            "value": 125401,
            "range": "± 545",
            "unit": "ns/iter"
          },
          {
            "name": "package_json_value",
            "value": 180582,
            "range": "± 1310",
            "unit": "ns/iter"
          },
          {
            "name": "tsconfig_json_ast",
            "value": 49727,
            "range": "± 247",
            "unit": "ns/iter"
          },
          {
            "name": "tsconfig_json_value",
            "value": 50573,
            "range": "± 150",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "dsherret@gmail.com",
            "name": "David Sherret",
            "username": "dsherret"
          },
          "committer": {
            "email": "dsherret@gmail.com",
            "name": "David Sherret",
            "username": "dsherret"
          },
          "distinct": true,
          "id": "c87687bd6fb2152dd95968d9bd874a9f22235d4d",
          "message": "chore(release): 0.17.0",
          "timestamp": "2021-04-03T11:43:21-04:00",
          "tree_id": "4469f77453192e25b27b63d1361ef38f025d5c13",
          "url": "https://github.com/dprint/jsonc-parser/commit/c87687bd6fb2152dd95968d9bd874a9f22235d4d"
        },
        "date": 1617464733965,
        "tool": "cargo",
        "benches": [
          {
            "name": "citm_catalog_json_large_ast",
            "value": 110132448,
            "range": "± 10213448",
            "unit": "ns/iter"
          },
          {
            "name": "citm_catalog_json_large_value",
            "value": 124952666,
            "range": "± 19342274",
            "unit": "ns/iter"
          },
          {
            "name": "package_json_ast",
            "value": 111009,
            "range": "± 17309",
            "unit": "ns/iter"
          },
          {
            "name": "package_json_value",
            "value": 149365,
            "range": "± 22465",
            "unit": "ns/iter"
          },
          {
            "name": "tsconfig_json_ast",
            "value": 48219,
            "range": "± 4161",
            "unit": "ns/iter"
          },
          {
            "name": "tsconfig_json_value",
            "value": 50594,
            "range": "± 1689",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "iwanabethatguy@qq.com",
            "name": "IWANABETHATGUY",
            "username": "IWANABETHATGUY"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "5e54cddfcb61816c021342bbc85e2d57f26e3111",
          "message": "Add link to benchmarks in contributing.md",
          "timestamp": "2021-04-07T17:15:24-04:00",
          "tree_id": "fa22a3c2a8c9705f8114e9de87fb5546553bc3bb",
          "url": "https://github.com/dprint/jsonc-parser/commit/5e54cddfcb61816c021342bbc85e2d57f26e3111"
        },
        "date": 1617830261298,
        "tool": "cargo",
        "benches": [
          {
            "name": "citm_catalog_json_large_ast",
            "value": 114844108,
            "range": "± 3291203",
            "unit": "ns/iter"
          },
          {
            "name": "citm_catalog_json_large_value",
            "value": 144583927,
            "range": "± 5630215",
            "unit": "ns/iter"
          },
          {
            "name": "package_json_ast",
            "value": 126652,
            "range": "± 13433",
            "unit": "ns/iter"
          },
          {
            "name": "package_json_value",
            "value": 177902,
            "range": "± 17429",
            "unit": "ns/iter"
          },
          {
            "name": "tsconfig_json_ast",
            "value": 47254,
            "range": "± 5884",
            "unit": "ns/iter"
          },
          {
            "name": "tsconfig_json_value",
            "value": 48435,
            "range": "± 5976",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "dsherret@gmail.com",
            "name": "David Sherret",
            "username": "dsherret"
          },
          "committer": {
            "email": "dsherret@gmail.com",
            "name": "David Sherret",
            "username": "dsherret"
          },
          "distinct": true,
          "id": "6ef505db6dd0331ab85bb386acfbcdda6a713f42",
          "message": "chore: Upgrade dprint and format code.",
          "timestamp": "2021-04-07T17:19:30-04:00",
          "tree_id": "04358839e8d9bd4511cf0a0a6b035aa9e2de516c",
          "url": "https://github.com/dprint/jsonc-parser/commit/6ef505db6dd0331ab85bb386acfbcdda6a713f42"
        },
        "date": 1617830508800,
        "tool": "cargo",
        "benches": [
          {
            "name": "citm_catalog_json_large_ast",
            "value": 107081402,
            "range": "± 6231692",
            "unit": "ns/iter"
          },
          {
            "name": "citm_catalog_json_large_value",
            "value": 129446922,
            "range": "± 8854199",
            "unit": "ns/iter"
          },
          {
            "name": "package_json_ast",
            "value": 116440,
            "range": "± 17785",
            "unit": "ns/iter"
          },
          {
            "name": "package_json_value",
            "value": 163892,
            "range": "± 22768",
            "unit": "ns/iter"
          },
          {
            "name": "tsconfig_json_ast",
            "value": 46344,
            "range": "± 6517",
            "unit": "ns/iter"
          },
          {
            "name": "tsconfig_json_value",
            "value": 44845,
            "range": "± 7804",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "dsherret@gmail.com",
            "name": "David Sherret",
            "username": "dsherret"
          },
          "committer": {
            "email": "dsherret@gmail.com",
            "name": "David Sherret",
            "username": "dsherret"
          },
          "distinct": true,
          "id": "ed1a18d05887e2e8ebf57b8a7692022566e9da1c",
          "message": "chore: Fix clippy errors.",
          "timestamp": "2021-05-09T13:43:43-04:00",
          "tree_id": "74f783ba372401c2ec474b0cc3c621114f870f1e",
          "url": "https://github.com/dprint/jsonc-parser/commit/ed1a18d05887e2e8ebf57b8a7692022566e9da1c"
        },
        "date": 1620582401557,
        "tool": "cargo",
        "benches": [
          {
            "name": "citm_catalog_json_large_ast",
            "value": 125348873,
            "range": "± 4020906",
            "unit": "ns/iter"
          },
          {
            "name": "citm_catalog_json_large_value",
            "value": 147543416,
            "range": "± 13348587",
            "unit": "ns/iter"
          },
          {
            "name": "package_json_ast",
            "value": 128159,
            "range": "± 14535",
            "unit": "ns/iter"
          },
          {
            "name": "package_json_value",
            "value": 177971,
            "range": "± 20253",
            "unit": "ns/iter"
          },
          {
            "name": "tsconfig_json_ast",
            "value": 45880,
            "range": "± 4172",
            "unit": "ns/iter"
          },
          {
            "name": "tsconfig_json_value",
            "value": 44283,
            "range": "± 7115",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "dsherret@gmail.com",
            "name": "David Sherret",
            "username": "dsherret"
          },
          "committer": {
            "email": "dsherret@gmail.com",
            "name": "David Sherret",
            "username": "dsherret"
          },
          "distinct": true,
          "id": "1a179350f0dad472b36adfbd57798b32ccaabeda",
          "message": "chore: Add serde to benchmark.",
          "timestamp": "2021-05-09T13:45:14-04:00",
          "tree_id": "d4fe24f8f78e9584b1240c383a22a377ba8043ce",
          "url": "https://github.com/dprint/jsonc-parser/commit/1a179350f0dad472b36adfbd57798b32ccaabeda"
        },
        "date": 1620582475960,
        "tool": "cargo",
        "benches": [
          {
            "name": "citm_catalog_json_large_ast",
            "value": 112280047,
            "range": "± 6792026",
            "unit": "ns/iter"
          },
          {
            "name": "citm_catalog_json_large_serde",
            "value": 47366479,
            "range": "± 3502937",
            "unit": "ns/iter"
          },
          {
            "name": "citm_catalog_json_large_value",
            "value": 127187646,
            "range": "± 9941204",
            "unit": "ns/iter"
          },
          {
            "name": "package_json_ast",
            "value": 120788,
            "range": "± 14607",
            "unit": "ns/iter"
          },
          {
            "name": "package_json_value",
            "value": 171557,
            "range": "± 30583",
            "unit": "ns/iter"
          },
          {
            "name": "tsconfig_json_ast",
            "value": 39077,
            "range": "± 4527",
            "unit": "ns/iter"
          },
          {
            "name": "tsconfig_json_value",
            "value": 41683,
            "range": "± 5467",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "dsherret@gmail.com",
            "name": "David Sherret",
            "username": "dsherret"
          },
          "committer": {
            "email": "dsherret@gmail.com",
            "name": "David Sherret",
            "username": "dsherret"
          },
          "distinct": true,
          "id": "1fa67f13dac0107ede63f650752d8b6a60525149",
          "message": "docs: document using \"serde\" feature for `parse_to_serde_value`\n\nCloses #20",
          "timestamp": "2021-12-11T12:17:37-05:00",
          "tree_id": "ff3a54088fbb6d3515bfa040f162ff276700bba8",
          "url": "https://github.com/dprint/jsonc-parser/commit/1fa67f13dac0107ede63f650752d8b6a60525149"
        },
        "date": 1639243212520,
        "tool": "cargo",
        "benches": [
          {
            "name": "citm_catalog_json_large_ast",
            "value": 97838298,
            "range": "± 720470",
            "unit": "ns/iter"
          },
          {
            "name": "citm_catalog_json_large_serde",
            "value": 42545833,
            "range": "± 1621400",
            "unit": "ns/iter"
          },
          {
            "name": "citm_catalog_json_large_value",
            "value": 113725566,
            "range": "± 2616545",
            "unit": "ns/iter"
          },
          {
            "name": "package_json_ast",
            "value": 96493,
            "range": "± 211",
            "unit": "ns/iter"
          },
          {
            "name": "package_json_value",
            "value": 139618,
            "range": "± 463",
            "unit": "ns/iter"
          },
          {
            "name": "tsconfig_json_ast",
            "value": 35958,
            "range": "± 111",
            "unit": "ns/iter"
          },
          {
            "name": "tsconfig_json_value",
            "value": 36666,
            "range": "± 89",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "dsherret@gmail.com",
            "name": "David Sherret",
            "username": "dsherret"
          },
          "committer": {
            "email": "dsherret@gmail.com",
            "name": "David Sherret",
            "username": "dsherret"
          },
          "distinct": true,
          "id": "3e52cb4acfc1b93d3b3349b2b465c3b82e384bb7",
          "message": "Format.",
          "timestamp": "2021-12-11T12:26:43-05:00",
          "tree_id": "334929894423ce21a578fe39488ef1b85a1ae9ef",
          "url": "https://github.com/dprint/jsonc-parser/commit/3e52cb4acfc1b93d3b3349b2b465c3b82e384bb7"
        },
        "date": 1639243751338,
        "tool": "cargo",
        "benches": [
          {
            "name": "citm_catalog_json_large_ast",
            "value": 117140013,
            "range": "± 12311459",
            "unit": "ns/iter"
          },
          {
            "name": "citm_catalog_json_large_serde",
            "value": 41415834,
            "range": "± 9166990",
            "unit": "ns/iter"
          },
          {
            "name": "citm_catalog_json_large_value",
            "value": 146429407,
            "range": "± 16816536",
            "unit": "ns/iter"
          },
          {
            "name": "package_json_ast",
            "value": 119684,
            "range": "± 14347",
            "unit": "ns/iter"
          },
          {
            "name": "package_json_value",
            "value": 168679,
            "range": "± 21137",
            "unit": "ns/iter"
          },
          {
            "name": "tsconfig_json_ast",
            "value": 46357,
            "range": "± 6824",
            "unit": "ns/iter"
          },
          {
            "name": "tsconfig_json_value",
            "value": 46406,
            "range": "± 7023",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "dsherret@gmail.com",
            "name": "David Sherret",
            "username": "dsherret"
          },
          "committer": {
            "email": "dsherret@gmail.com",
            "name": "David Sherret",
            "username": "dsherret"
          },
          "distinct": true,
          "id": "1de642cbc48e22845b7e01fee6a2e139af9d1ba3",
          "message": "chore: format - use indent width of 2",
          "timestamp": "2021-12-11T12:35:04-05:00",
          "tree_id": "fb964bd8ff29d0c71ca57cd22b5f4924ab775d0c",
          "url": "https://github.com/dprint/jsonc-parser/commit/1de642cbc48e22845b7e01fee6a2e139af9d1ba3"
        },
        "date": 1639244254776,
        "tool": "cargo",
        "benches": [
          {
            "name": "citm_catalog_json_large_ast",
            "value": 121401815,
            "range": "± 7271843",
            "unit": "ns/iter"
          },
          {
            "name": "citm_catalog_json_large_serde",
            "value": 46095649,
            "range": "± 6505163",
            "unit": "ns/iter"
          },
          {
            "name": "citm_catalog_json_large_value",
            "value": 143431280,
            "range": "± 12551876",
            "unit": "ns/iter"
          },
          {
            "name": "package_json_ast",
            "value": 119810,
            "range": "± 14691",
            "unit": "ns/iter"
          },
          {
            "name": "package_json_value",
            "value": 167788,
            "range": "± 15859",
            "unit": "ns/iter"
          },
          {
            "name": "tsconfig_json_ast",
            "value": 45192,
            "range": "± 11225",
            "unit": "ns/iter"
          },
          {
            "name": "tsconfig_json_value",
            "value": 46371,
            "range": "± 8173",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "dsherret@gmail.com",
            "name": "David Sherret",
            "username": "dsherret"
          },
          "committer": {
            "email": "dsherret@gmail.com",
            "name": "David Sherret",
            "username": "dsherret"
          },
          "distinct": true,
          "id": "515b6f18920f498870d36f274dcf0121e178ae0b",
          "message": "chore: fix unicode test",
          "timestamp": "2021-12-11T12:38:53-05:00",
          "tree_id": "d6421064ebc1e8ab695399265dc121f664aeaca6",
          "url": "https://github.com/dprint/jsonc-parser/commit/515b6f18920f498870d36f274dcf0121e178ae0b"
        },
        "date": 1639244483001,
        "tool": "cargo",
        "benches": [
          {
            "name": "citm_catalog_json_large_ast",
            "value": 114997615,
            "range": "± 8939459",
            "unit": "ns/iter"
          },
          {
            "name": "citm_catalog_json_large_serde",
            "value": 53865015,
            "range": "± 3693593",
            "unit": "ns/iter"
          },
          {
            "name": "citm_catalog_json_large_value",
            "value": 135840802,
            "range": "± 8853060",
            "unit": "ns/iter"
          },
          {
            "name": "package_json_ast",
            "value": 116149,
            "range": "± 22753",
            "unit": "ns/iter"
          },
          {
            "name": "package_json_value",
            "value": 168640,
            "range": "± 54778",
            "unit": "ns/iter"
          },
          {
            "name": "tsconfig_json_ast",
            "value": 43162,
            "range": "± 7860",
            "unit": "ns/iter"
          },
          {
            "name": "tsconfig_json_value",
            "value": 44170,
            "range": "± 8825",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "dsherret@gmail.com",
            "name": "David Sherret",
            "username": "dsherret"
          },
          "committer": {
            "email": "dsherret@gmail.com",
            "name": "David Sherret",
            "username": "dsherret"
          },
          "distinct": true,
          "id": "ca3476358b743c0f92c9d9d129fb2b1aea3e647b",
          "message": "fix: correct end position of error range when at multi-byte character",
          "timestamp": "2021-12-11T13:09:29-05:00",
          "tree_id": "293c810977979d575ee1a6d756643f24aca6f9c1",
          "url": "https://github.com/dprint/jsonc-parser/commit/ca3476358b743c0f92c9d9d129fb2b1aea3e647b"
        },
        "date": 1639248363855,
        "tool": "cargo",
        "benches": [
          {
            "name": "citm_catalog_json_large_ast",
            "value": 119002960,
            "range": "± 799522",
            "unit": "ns/iter"
          },
          {
            "name": "citm_catalog_json_large_serde",
            "value": 51155277,
            "range": "± 1704271",
            "unit": "ns/iter"
          },
          {
            "name": "citm_catalog_json_large_value",
            "value": 137521629,
            "range": "± 1092916",
            "unit": "ns/iter"
          },
          {
            "name": "package_json_ast",
            "value": 120243,
            "range": "± 979",
            "unit": "ns/iter"
          },
          {
            "name": "package_json_value",
            "value": 174437,
            "range": "± 964",
            "unit": "ns/iter"
          },
          {
            "name": "tsconfig_json_ast",
            "value": 44580,
            "range": "± 184",
            "unit": "ns/iter"
          },
          {
            "name": "tsconfig_json_value",
            "value": 45458,
            "range": "± 260",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "dsherret@gmail.com",
            "name": "David Sherret",
            "username": "dsherret"
          },
          "committer": {
            "email": "dsherret@gmail.com",
            "name": "David Sherret",
            "username": "dsherret"
          },
          "distinct": true,
          "id": "b1977115bf8ff12c50eee5b1343979f53885b241",
          "message": "chore: fix CI",
          "timestamp": "2021-12-11T13:47:43-05:00",
          "tree_id": "222aad3abca982e9ad5304fb65ee0dd155dd8351",
          "url": "https://github.com/dprint/jsonc-parser/commit/b1977115bf8ff12c50eee5b1343979f53885b241"
        },
        "date": 1639248581054,
        "tool": "cargo",
        "benches": [
          {
            "name": "citm_catalog_json_large_ast",
            "value": 97291534,
            "range": "± 406222",
            "unit": "ns/iter"
          },
          {
            "name": "citm_catalog_json_large_serde",
            "value": 42270730,
            "range": "± 1810612",
            "unit": "ns/iter"
          },
          {
            "name": "citm_catalog_json_large_value",
            "value": 112565842,
            "range": "± 1386770",
            "unit": "ns/iter"
          },
          {
            "name": "package_json_ast",
            "value": 96056,
            "range": "± 168",
            "unit": "ns/iter"
          },
          {
            "name": "package_json_value",
            "value": 138850,
            "range": "± 317",
            "unit": "ns/iter"
          },
          {
            "name": "tsconfig_json_ast",
            "value": 35697,
            "range": "± 261",
            "unit": "ns/iter"
          },
          {
            "name": "tsconfig_json_value",
            "value": 36370,
            "range": "± 67",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "dsherret@users.noreply.github.com",
            "name": "David Sherret",
            "username": "dsherret"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "65cccfcec4685c26c39fd1a45a6ed8c0596b5257",
          "message": "refactor: store ranges as a start and end position (#22)",
          "timestamp": "2022-01-16T18:29:36-05:00",
          "tree_id": "5fa6168fa0434c292061633aa25137c80d69ac42",
          "url": "https://github.com/dprint/jsonc-parser/commit/65cccfcec4685c26c39fd1a45a6ed8c0596b5257"
        },
        "date": 1642375952572,
        "tool": "cargo",
        "benches": [
          {
            "name": "citm_catalog_json_large_ast",
            "value": 117773096,
            "range": "± 19542289",
            "unit": "ns/iter"
          },
          {
            "name": "citm_catalog_json_large_serde",
            "value": 38256708,
            "range": "± 7884190",
            "unit": "ns/iter"
          },
          {
            "name": "citm_catalog_json_large_value",
            "value": 137606473,
            "range": "± 18498329",
            "unit": "ns/iter"
          },
          {
            "name": "package_json_ast",
            "value": 116735,
            "range": "± 26766",
            "unit": "ns/iter"
          },
          {
            "name": "package_json_value",
            "value": 169905,
            "range": "± 41972",
            "unit": "ns/iter"
          },
          {
            "name": "tsconfig_json_ast",
            "value": 38886,
            "range": "± 9324",
            "unit": "ns/iter"
          },
          {
            "name": "tsconfig_json_value",
            "value": 41426,
            "range": "± 7512",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "dsherret@users.noreply.github.com",
            "name": "David Sherret",
            "username": "dsherret"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "e2c0e2292325fe56cbbd381992079994abb4d859",
          "message": "refactor: remove line numbers from `Range` (#24)",
          "timestamp": "2022-01-29T18:12:12-05:00",
          "tree_id": "735b409c2288ee6805bfeb9d23f88150fb627043",
          "url": "https://github.com/dprint/jsonc-parser/commit/e2c0e2292325fe56cbbd381992079994abb4d859"
        },
        "date": 1643498080516,
        "tool": "cargo",
        "benches": [
          {
            "name": "citm_catalog_json_large_ast",
            "value": 93005067,
            "range": "± 10833884",
            "unit": "ns/iter"
          },
          {
            "name": "citm_catalog_json_large_serde",
            "value": 45646235,
            "range": "± 7623538",
            "unit": "ns/iter"
          },
          {
            "name": "citm_catalog_json_large_value",
            "value": 131160139,
            "range": "± 18823083",
            "unit": "ns/iter"
          },
          {
            "name": "package_json_ast",
            "value": 113211,
            "range": "± 28335",
            "unit": "ns/iter"
          },
          {
            "name": "package_json_value",
            "value": 156301,
            "range": "± 20378",
            "unit": "ns/iter"
          },
          {
            "name": "tsconfig_json_ast",
            "value": 36512,
            "range": "± 5565",
            "unit": "ns/iter"
          },
          {
            "name": "tsconfig_json_value",
            "value": 37610,
            "range": "± 7487",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "dsherret@gmail.com",
            "name": "David Sherret",
            "username": "dsherret"
          },
          "committer": {
            "email": "dsherret@gmail.com",
            "name": "David Sherret",
            "username": "dsherret"
          },
          "distinct": true,
          "id": "c67569a2dc9dc6f061a29ea32ca5ccb165aa4ea9",
          "message": "chore(release): 0.19.0",
          "timestamp": "2022-01-29T18:13:52-05:00",
          "tree_id": "deaba08cff40c94513252c6c132b59cf206d359b",
          "url": "https://github.com/dprint/jsonc-parser/commit/c67569a2dc9dc6f061a29ea32ca5ccb165aa4ea9"
        },
        "date": 1643498174119,
        "tool": "cargo",
        "benches": [
          {
            "name": "citm_catalog_json_large_ast",
            "value": 99284160,
            "range": "± 1281981",
            "unit": "ns/iter"
          },
          {
            "name": "citm_catalog_json_large_serde",
            "value": 46608168,
            "range": "± 2729101",
            "unit": "ns/iter"
          },
          {
            "name": "citm_catalog_json_large_value",
            "value": 137491375,
            "range": "± 3397090",
            "unit": "ns/iter"
          },
          {
            "name": "package_json_ast",
            "value": 126991,
            "range": "± 550",
            "unit": "ns/iter"
          },
          {
            "name": "package_json_value",
            "value": 179275,
            "range": "± 3809",
            "unit": "ns/iter"
          },
          {
            "name": "tsconfig_json_ast",
            "value": 40773,
            "range": "± 1489",
            "unit": "ns/iter"
          },
          {
            "name": "tsconfig_json_value",
            "value": 41624,
            "range": "± 1984",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "dsherret@users.noreply.github.com",
            "name": "David Sherret",
            "username": "dsherret"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "572c86a55287525959e6729aaa98c0d17528a558",
          "message": "feat: ability to parse strictly as JSON (#25)",
          "timestamp": "2022-07-26T20:42:00-04:00",
          "tree_id": "cf9b93346b522437b3987cc44ed6a4436fb822d7",
          "url": "https://github.com/dprint/jsonc-parser/commit/572c86a55287525959e6729aaa98c0d17528a558"
        },
        "date": 1658882647874,
        "tool": "cargo",
        "benches": [
          {
            "name": "citm_catalog_json_large_ast",
            "value": 76485517,
            "range": "± 1054268",
            "unit": "ns/iter"
          },
          {
            "name": "citm_catalog_json_large_serde",
            "value": 44248587,
            "range": "± 4427561",
            "unit": "ns/iter"
          },
          {
            "name": "citm_catalog_json_large_value",
            "value": 113290387,
            "range": "± 3894298",
            "unit": "ns/iter"
          },
          {
            "name": "package_json_ast",
            "value": 99426,
            "range": "± 201",
            "unit": "ns/iter"
          },
          {
            "name": "package_json_value",
            "value": 148435,
            "range": "± 827",
            "unit": "ns/iter"
          },
          {
            "name": "tsconfig_json_ast",
            "value": 41482,
            "range": "± 156",
            "unit": "ns/iter"
          },
          {
            "name": "tsconfig_json_value",
            "value": 42287,
            "range": "± 109",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "dsherret@gmail.com",
            "name": "David Sherret",
            "username": "dsherret"
          },
          "committer": {
            "email": "dsherret@gmail.com",
            "name": "David Sherret",
            "username": "dsherret"
          },
          "distinct": true,
          "id": "bfee0d972fb3a002568f60de8b224ce72ebef2fb",
          "message": "0.20.0",
          "timestamp": "2022-07-26T20:42:45-04:00",
          "tree_id": "ccb7de14ff9eb3a9910f07182ee21c1b7cda13e3",
          "url": "https://github.com/dprint/jsonc-parser/commit/bfee0d972fb3a002568f60de8b224ce72ebef2fb"
        },
        "date": 1658882714864,
        "tool": "cargo",
        "benches": [
          {
            "name": "citm_catalog_json_large_ast",
            "value": 75334603,
            "range": "± 1489866",
            "unit": "ns/iter"
          },
          {
            "name": "citm_catalog_json_large_serde",
            "value": 43475152,
            "range": "± 3896572",
            "unit": "ns/iter"
          },
          {
            "name": "citm_catalog_json_large_value",
            "value": 111200857,
            "range": "± 3196042",
            "unit": "ns/iter"
          },
          {
            "name": "package_json_ast",
            "value": 95305,
            "range": "± 152",
            "unit": "ns/iter"
          },
          {
            "name": "package_json_value",
            "value": 141818,
            "range": "± 1380",
            "unit": "ns/iter"
          },
          {
            "name": "tsconfig_json_ast",
            "value": 36036,
            "range": "± 119",
            "unit": "ns/iter"
          },
          {
            "name": "tsconfig_json_value",
            "value": 37089,
            "range": "± 258",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "me@kitsonkelly.com",
            "name": "Kitson Kelly",
            "username": "kitsonk"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "e6525febaa079bc47619fd8d94664f2921aa95d3",
          "message": "feat: add from trait for serde value (#30)\n\nCloses: #26",
          "timestamp": "2022-08-04T22:48:37-04:00",
          "tree_id": "e83043fca6691d4555a2b5cfd541c49b7ea8416a",
          "url": "https://github.com/dprint/jsonc-parser/commit/e6525febaa079bc47619fd8d94664f2921aa95d3"
        },
        "date": 1659667853981,
        "tool": "cargo",
        "benches": [
          {
            "name": "citm_catalog_json_large_ast",
            "value": 75888939,
            "range": "± 1762170",
            "unit": "ns/iter"
          },
          {
            "name": "citm_catalog_json_large_serde",
            "value": 39424292,
            "range": "± 523120",
            "unit": "ns/iter"
          },
          {
            "name": "citm_catalog_json_large_value",
            "value": 110079252,
            "range": "± 5572269",
            "unit": "ns/iter"
          },
          {
            "name": "package_json_ast",
            "value": 99905,
            "range": "± 1188",
            "unit": "ns/iter"
          },
          {
            "name": "package_json_value",
            "value": 145550,
            "range": "± 757",
            "unit": "ns/iter"
          },
          {
            "name": "tsconfig_json_ast",
            "value": 32262,
            "range": "± 429",
            "unit": "ns/iter"
          },
          {
            "name": "tsconfig_json_value",
            "value": 32990,
            "range": "± 502",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "dsherret@gmail.com",
            "name": "David Sherret",
            "username": "dsherret"
          },
          "committer": {
            "email": "dsherret@gmail.com",
            "name": "David Sherret",
            "username": "dsherret"
          },
          "distinct": true,
          "id": "085f0be153c608f826de9988f29c10573b3bc05f",
          "message": "0.21.0",
          "timestamp": "2022-08-04T22:49:57-04:00",
          "tree_id": "f6119dda768ded197ababcaf7b74ffa480ce1bbb",
          "url": "https://github.com/dprint/jsonc-parser/commit/085f0be153c608f826de9988f29c10573b3bc05f"
        },
        "date": 1659667950183,
        "tool": "cargo",
        "benches": [
          {
            "name": "citm_catalog_json_large_ast",
            "value": 78106255,
            "range": "± 8103377",
            "unit": "ns/iter"
          },
          {
            "name": "citm_catalog_json_large_serde",
            "value": 42131405,
            "range": "± 7618658",
            "unit": "ns/iter"
          },
          {
            "name": "citm_catalog_json_large_value",
            "value": 113887610,
            "range": "± 13814838",
            "unit": "ns/iter"
          },
          {
            "name": "package_json_ast",
            "value": 98096,
            "range": "± 25716",
            "unit": "ns/iter"
          },
          {
            "name": "package_json_value",
            "value": 148693,
            "range": "± 24663",
            "unit": "ns/iter"
          },
          {
            "name": "tsconfig_json_ast",
            "value": 33849,
            "range": "± 8382",
            "unit": "ns/iter"
          },
          {
            "name": "tsconfig_json_value",
            "value": 36052,
            "range": "± 8277",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "dsherret@users.noreply.github.com",
            "name": "David Sherret",
            "username": "dsherret"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "f2cee75bb8d3878cf99a479c21d7d1769fb6ccac",
          "message": "fix: support parsing exponent without plus or minus sign (#32)",
          "timestamp": "2023-04-28T10:50:02-04:00",
          "tree_id": "47edceacea14c0b735524beedc826a482dc74a60",
          "url": "https://github.com/dprint/jsonc-parser/commit/f2cee75bb8d3878cf99a479c21d7d1769fb6ccac"
        },
        "date": 1682693506007,
        "tool": "cargo",
        "benches": [
          {
            "name": "citm_catalog_json_large_ast",
            "value": 61804242,
            "range": "± 1658803",
            "unit": "ns/iter"
          },
          {
            "name": "citm_catalog_json_large_serde",
            "value": 34351456,
            "range": "± 1946537",
            "unit": "ns/iter"
          },
          {
            "name": "citm_catalog_json_large_value",
            "value": 94128610,
            "range": "± 3308684",
            "unit": "ns/iter"
          },
          {
            "name": "package_json_ast",
            "value": 85323,
            "range": "± 185",
            "unit": "ns/iter"
          },
          {
            "name": "package_json_value",
            "value": 132853,
            "range": "± 281",
            "unit": "ns/iter"
          },
          {
            "name": "tsconfig_json_ast",
            "value": 36025,
            "range": "± 70",
            "unit": "ns/iter"
          },
          {
            "name": "tsconfig_json_value",
            "value": 36854,
            "range": "± 79",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "dsherret@gmail.com",
            "name": "David Sherret",
            "username": "dsherret"
          },
          "committer": {
            "email": "dsherret@gmail.com",
            "name": "David Sherret",
            "username": "dsherret"
          },
          "distinct": true,
          "id": "10793bc2cd72b4296d3ce6384edb828f149be2b5",
          "message": "ci: add release workflow",
          "timestamp": "2023-04-28T10:52:17-04:00",
          "tree_id": "ee6ea3c8f19a30cb675d9fa82564fb7324ae5c72",
          "url": "https://github.com/dprint/jsonc-parser/commit/10793bc2cd72b4296d3ce6384edb828f149be2b5"
        },
        "date": 1682693691025,
        "tool": "cargo",
        "benches": [
          {
            "name": "citm_catalog_json_large_ast",
            "value": 79989986,
            "range": "± 4076456",
            "unit": "ns/iter"
          },
          {
            "name": "citm_catalog_json_large_serde",
            "value": 48557103,
            "range": "± 2070115",
            "unit": "ns/iter"
          },
          {
            "name": "citm_catalog_json_large_value",
            "value": 124182328,
            "range": "± 4341197",
            "unit": "ns/iter"
          },
          {
            "name": "package_json_ast",
            "value": 100463,
            "range": "± 5550",
            "unit": "ns/iter"
          },
          {
            "name": "package_json_value",
            "value": 156448,
            "range": "± 10575",
            "unit": "ns/iter"
          },
          {
            "name": "tsconfig_json_ast",
            "value": 40385,
            "range": "± 2141",
            "unit": "ns/iter"
          },
          {
            "name": "tsconfig_json_value",
            "value": 41129,
            "range": "± 2274",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "dprintbot@users.noreply.github.com",
            "name": "dprintbot",
            "username": "dprintbot"
          },
          "committer": {
            "email": "dprintbot@users.noreply.github.com",
            "name": "dprintbot",
            "username": "dprintbot"
          },
          "distinct": true,
          "id": "029a1fe226c07376dc3913ce10d3dbd823c23e18",
          "message": "0.21.1",
          "timestamp": "2023-04-28T14:57:49Z",
          "tree_id": "730aa1d766918b23d5b104904736137cb79e7427",
          "url": "https://github.com/dprint/jsonc-parser/commit/029a1fe226c07376dc3913ce10d3dbd823c23e18"
        },
        "date": 1682693984860,
        "tool": "cargo",
        "benches": [
          {
            "name": "citm_catalog_json_large_ast",
            "value": 65586059,
            "range": "± 773606",
            "unit": "ns/iter"
          },
          {
            "name": "citm_catalog_json_large_serde",
            "value": 41979221,
            "range": "± 994541",
            "unit": "ns/iter"
          },
          {
            "name": "citm_catalog_json_large_value",
            "value": 102210682,
            "range": "± 4280378",
            "unit": "ns/iter"
          },
          {
            "name": "package_json_ast",
            "value": 86993,
            "range": "± 212",
            "unit": "ns/iter"
          },
          {
            "name": "package_json_value",
            "value": 133674,
            "range": "± 428",
            "unit": "ns/iter"
          },
          {
            "name": "tsconfig_json_ast",
            "value": 36015,
            "range": "± 87",
            "unit": "ns/iter"
          },
          {
            "name": "tsconfig_json_value",
            "value": 36801,
            "range": "± 73",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "dsherret@users.noreply.github.com",
            "name": "David Sherret",
            "username": "dsherret"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "7397ef38e4dca7ff18897e5a103d14164be84d69",
          "message": "chore: upgrade ci caching (#34)",
          "timestamp": "2023-07-18T11:23:05-04:00",
          "tree_id": "496d05b239e138298e7d154b5a72b55451b26ddb",
          "url": "https://github.com/dprint/jsonc-parser/commit/7397ef38e4dca7ff18897e5a103d14164be84d69"
        },
        "date": 1689693897855,
        "tool": "cargo",
        "benches": [
          {
            "name": "citm_catalog_json_large_ast",
            "value": 65843455,
            "range": "± 812752",
            "unit": "ns/iter"
          },
          {
            "name": "citm_catalog_json_large_serde",
            "value": 44745576,
            "range": "± 2947620",
            "unit": "ns/iter"
          },
          {
            "name": "citm_catalog_json_large_value",
            "value": 103985438,
            "range": "± 4281914",
            "unit": "ns/iter"
          },
          {
            "name": "package_json_ast",
            "value": 87035,
            "range": "± 203",
            "unit": "ns/iter"
          },
          {
            "name": "package_json_value",
            "value": 134494,
            "range": "± 347",
            "unit": "ns/iter"
          },
          {
            "name": "tsconfig_json_ast",
            "value": 38925,
            "range": "± 240",
            "unit": "ns/iter"
          },
          {
            "name": "tsconfig_json_value",
            "value": 39846,
            "range": "± 267",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "dsherret@users.noreply.github.com",
            "name": "David Sherret",
            "username": "dsherret"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "e8ccdcf739526ae6d008a3cffa0f5ae8f3ac231f",
          "message": "feat: `Node` - add `as_x` helper properties (#35)",
          "timestamp": "2023-07-27T18:52:16-04:00",
          "tree_id": "1b0c97457b7bd86d8276b27836ceb66233ad0fb4",
          "url": "https://github.com/dprint/jsonc-parser/commit/e8ccdcf739526ae6d008a3cffa0f5ae8f3ac231f"
        },
        "date": 1690498445054,
        "tool": "cargo",
        "benches": [
          {
            "name": "citm_catalog_json_large_ast",
            "value": 57014214,
            "range": "± 7416427",
            "unit": "ns/iter"
          },
          {
            "name": "citm_catalog_json_large_serde",
            "value": 31456583,
            "range": "± 6043704",
            "unit": "ns/iter"
          },
          {
            "name": "citm_catalog_json_large_value",
            "value": 89548268,
            "range": "± 11783131",
            "unit": "ns/iter"
          },
          {
            "name": "package_json_ast",
            "value": 75432,
            "range": "± 12940",
            "unit": "ns/iter"
          },
          {
            "name": "package_json_value",
            "value": 115493,
            "range": "± 25305",
            "unit": "ns/iter"
          },
          {
            "name": "tsconfig_json_ast",
            "value": 32067,
            "range": "± 3594",
            "unit": "ns/iter"
          },
          {
            "name": "tsconfig_json_value",
            "value": 33237,
            "range": "± 5858",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "dprintbot@users.noreply.github.com",
            "name": "dprintbot",
            "username": "dprintbot"
          },
          "committer": {
            "email": "dprintbot@users.noreply.github.com",
            "name": "dprintbot",
            "username": "dprintbot"
          },
          "distinct": true,
          "id": "47e81eccefd32ad829db05487947adbab881e189",
          "message": "0.22.0",
          "timestamp": "2023-07-27T22:53:07Z",
          "tree_id": "36346788bbc0feb85493dc52fa743f527f3f986a",
          "url": "https://github.com/dprint/jsonc-parser/commit/47e81eccefd32ad829db05487947adbab881e189"
        },
        "date": 1690498494134,
        "tool": "cargo",
        "benches": [
          {
            "name": "citm_catalog_json_large_ast",
            "value": 59887461,
            "range": "± 2350430",
            "unit": "ns/iter"
          },
          {
            "name": "citm_catalog_json_large_serde",
            "value": 29776257,
            "range": "± 3960175",
            "unit": "ns/iter"
          },
          {
            "name": "citm_catalog_json_large_value",
            "value": 90711338,
            "range": "± 6672625",
            "unit": "ns/iter"
          },
          {
            "name": "package_json_ast",
            "value": 85045,
            "range": "± 1286",
            "unit": "ns/iter"
          },
          {
            "name": "package_json_value",
            "value": 139010,
            "range": "± 1209",
            "unit": "ns/iter"
          },
          {
            "name": "tsconfig_json_ast",
            "value": 34469,
            "range": "± 347",
            "unit": "ns/iter"
          },
          {
            "name": "tsconfig_json_value",
            "value": 35246,
            "range": "± 643",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "dprintbot@users.noreply.github.com",
            "name": "dprintbot",
            "username": "dprintbot"
          },
          "committer": {
            "email": "dprintbot@users.noreply.github.com",
            "name": "dprintbot",
            "username": "dprintbot"
          },
          "distinct": true,
          "id": "abd65a888339815d3a8c80359ddc918c2b4c25c2",
          "message": "0.22.1",
          "timestamp": "2023-07-27T23:08:37Z",
          "tree_id": "115e0963c02bde2587cfba66b918248d36397646",
          "url": "https://github.com/dprint/jsonc-parser/commit/abd65a888339815d3a8c80359ddc918c2b4c25c2"
        },
        "date": 1690499414591,
        "tool": "cargo",
        "benches": [
          {
            "name": "citm_catalog_json_large_ast",
            "value": 59691726,
            "range": "± 1603273",
            "unit": "ns/iter"
          },
          {
            "name": "citm_catalog_json_large_serde",
            "value": 30615159,
            "range": "± 3426332",
            "unit": "ns/iter"
          },
          {
            "name": "citm_catalog_json_large_value",
            "value": 91599489,
            "range": "± 3087044",
            "unit": "ns/iter"
          },
          {
            "name": "package_json_ast",
            "value": 83478,
            "range": "± 547",
            "unit": "ns/iter"
          },
          {
            "name": "package_json_value",
            "value": 132695,
            "range": "± 736",
            "unit": "ns/iter"
          },
          {
            "name": "tsconfig_json_ast",
            "value": 33442,
            "range": "± 633",
            "unit": "ns/iter"
          },
          {
            "name": "tsconfig_json_value",
            "value": 34464,
            "range": "± 486",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "dsherret@users.noreply.github.com",
            "name": "David Sherret",
            "username": "dsherret"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "02dcfd148a00c1ca84b575f907ae40e2ab277f03",
          "message": "fix: correct lifetimes on helper return values (#36)",
          "timestamp": "2023-07-27T19:07:58-04:00",
          "tree_id": "2eab49614b8c4186721b6736f636d3dd983c7430",
          "url": "https://github.com/dprint/jsonc-parser/commit/02dcfd148a00c1ca84b575f907ae40e2ab277f03"
        },
        "date": 1690499421550,
        "tool": "cargo",
        "benches": [
          {
            "name": "citm_catalog_json_large_ast",
            "value": 83231926,
            "range": "± 7526417",
            "unit": "ns/iter"
          },
          {
            "name": "citm_catalog_json_large_serde",
            "value": 53731027,
            "range": "± 5240190",
            "unit": "ns/iter"
          },
          {
            "name": "citm_catalog_json_large_value",
            "value": 130506219,
            "range": "± 12079193",
            "unit": "ns/iter"
          },
          {
            "name": "package_json_ast",
            "value": 101953,
            "range": "± 12577",
            "unit": "ns/iter"
          },
          {
            "name": "package_json_value",
            "value": 154940,
            "range": "± 19971",
            "unit": "ns/iter"
          },
          {
            "name": "tsconfig_json_ast",
            "value": 40729,
            "range": "± 9160",
            "unit": "ns/iter"
          },
          {
            "name": "tsconfig_json_value",
            "value": 41830,
            "range": "± 6463",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "dsherret@users.noreply.github.com",
            "name": "David Sherret",
            "username": "dsherret"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "c4eaf5bcea675dd3ed9f0f2145326bb31744f6bb",
          "message": "feat: helper methods on Value (#37)",
          "timestamp": "2023-10-27T01:19:23-04:00",
          "tree_id": "aee5d2df8ab41404b8a9ae3a9f0e661bdf6634cc",
          "url": "https://github.com/dprint/jsonc-parser/commit/c4eaf5bcea675dd3ed9f0f2145326bb31744f6bb"
        },
        "date": 1698384079822,
        "tool": "cargo",
        "benches": [
          {
            "name": "citm_catalog_json_large_ast",
            "value": 86609963,
            "range": "± 477229",
            "unit": "ns/iter"
          },
          {
            "name": "citm_catalog_json_large_serde",
            "value": 31694514,
            "range": "± 2562472",
            "unit": "ns/iter"
          },
          {
            "name": "citm_catalog_json_large_value",
            "value": 118087739,
            "range": "± 3497509",
            "unit": "ns/iter"
          },
          {
            "name": "package_json_ast",
            "value": 111356,
            "range": "± 1764",
            "unit": "ns/iter"
          },
          {
            "name": "package_json_value",
            "value": 155822,
            "range": "± 1003",
            "unit": "ns/iter"
          },
          {
            "name": "tsconfig_json_ast",
            "value": 36133,
            "range": "± 712",
            "unit": "ns/iter"
          },
          {
            "name": "tsconfig_json_value",
            "value": 37109,
            "range": "± 545",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "dsherret@users.noreply.github.com",
            "name": "David Sherret",
            "username": "dsherret"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "2c70e2aaffc4ec29f979120ac3fa3d9f6e1d34c5",
          "message": "feat: add preserve_order feature (#39)",
          "timestamp": "2023-10-27T01:21:09-04:00",
          "tree_id": "24b8ae1f678199a220ae699cacd95c4ece8b8d9d",
          "url": "https://github.com/dprint/jsonc-parser/commit/2c70e2aaffc4ec29f979120ac3fa3d9f6e1d34c5"
        },
        "date": 1698384182026,
        "tool": "cargo",
        "benches": [
          {
            "name": "citm_catalog_json_large_ast",
            "value": 80783441,
            "range": "± 1193132",
            "unit": "ns/iter"
          },
          {
            "name": "citm_catalog_json_large_serde",
            "value": 46032092,
            "range": "± 2565685",
            "unit": "ns/iter"
          },
          {
            "name": "citm_catalog_json_large_value",
            "value": 113197127,
            "range": "± 2015912",
            "unit": "ns/iter"
          },
          {
            "name": "package_json_ast",
            "value": 101955,
            "range": "± 178",
            "unit": "ns/iter"
          },
          {
            "name": "package_json_value",
            "value": 130192,
            "range": "± 409",
            "unit": "ns/iter"
          },
          {
            "name": "tsconfig_json_ast",
            "value": 37755,
            "range": "± 81",
            "unit": "ns/iter"
          },
          {
            "name": "tsconfig_json_value",
            "value": 38405,
            "range": "± 94",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "dprintbot@users.noreply.github.com",
            "name": "dprintbot",
            "username": "dprintbot"
          },
          "committer": {
            "email": "dprintbot@users.noreply.github.com",
            "name": "dprintbot",
            "username": "dprintbot"
          },
          "distinct": true,
          "id": "a3690d0608a0805227bd393aa154e3975d7e2536",
          "message": "0.23.0",
          "timestamp": "2023-10-27T05:21:52Z",
          "tree_id": "85bf4b430822ffd64c1a8fe778eb5775d1886e62",
          "url": "https://github.com/dprint/jsonc-parser/commit/a3690d0608a0805227bd393aa154e3975d7e2536"
        },
        "date": 1698384238300,
        "tool": "cargo",
        "benches": [
          {
            "name": "citm_catalog_json_large_ast",
            "value": 93279521,
            "range": "± 2790220",
            "unit": "ns/iter"
          },
          {
            "name": "citm_catalog_json_large_serde",
            "value": 43676719,
            "range": "± 1656392",
            "unit": "ns/iter"
          },
          {
            "name": "citm_catalog_json_large_value",
            "value": 128732625,
            "range": "± 4560152",
            "unit": "ns/iter"
          },
          {
            "name": "package_json_ast",
            "value": 117476,
            "range": "± 17248",
            "unit": "ns/iter"
          },
          {
            "name": "package_json_value",
            "value": 168828,
            "range": "± 25218",
            "unit": "ns/iter"
          },
          {
            "name": "tsconfig_json_ast",
            "value": 43183,
            "range": "± 6181",
            "unit": "ns/iter"
          },
          {
            "name": "tsconfig_json_value",
            "value": 45126,
            "range": "± 7938",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "dsherret@users.noreply.github.com",
            "name": "David Sherret",
            "username": "dsherret"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "3c9eab0582f90634dbcea1a3cfc8c46381456d45",
          "message": "feat: CST and first class manipulation API (#41)",
          "timestamp": "2024-10-19T11:50:00-04:00",
          "tree_id": "37a18a9010d3b3526ddadc2a7e682c67742e6106",
          "url": "https://github.com/dprint/jsonc-parser/commit/3c9eab0582f90634dbcea1a3cfc8c46381456d45"
        },
        "date": 1729353076224,
        "tool": "cargo",
        "benches": [
          {
            "name": "citm_catalog_json_large_ast",
            "value": 44085270.1,
            "range": "± 1426970.88",
            "unit": "ns/iter"
          },
          {
            "name": "citm_catalog_json_large_serde",
            "value": 27517360.2,
            "range": "± 1782209.86",
            "unit": "ns/iter"
          },
          {
            "name": "citm_catalog_json_large_value",
            "value": 66689690.1,
            "range": "± 3132768.44",
            "unit": "ns/iter"
          },
          {
            "name": "package_json_ast",
            "value": 64476.49,
            "range": "± 1625.39",
            "unit": "ns/iter"
          },
          {
            "name": "package_json_value",
            "value": 83835.24,
            "range": "± 754.49",
            "unit": "ns/iter"
          },
          {
            "name": "tsconfig_json_ast",
            "value": 23328.59,
            "range": "± 212.23",
            "unit": "ns/iter"
          },
          {
            "name": "tsconfig_json_value",
            "value": 23791.61,
            "range": "± 159.10",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "dsherret@users.noreply.github.com",
            "name": "David Sherret",
            "username": "dsherret"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "438b362104c8852085a912d5fa31800b0dc93aff",
          "message": "feat: improve error struct (#42)",
          "timestamp": "2024-10-19T13:21:30-04:00",
          "tree_id": "4870949e41b6fc9a93c90201c76fe244dbe5de68",
          "url": "https://github.com/dprint/jsonc-parser/commit/438b362104c8852085a912d5fa31800b0dc93aff"
        },
        "date": 1729358567950,
        "tool": "cargo",
        "benches": [
          {
            "name": "citm_catalog_json_large_ast",
            "value": 44137449.2,
            "range": "± 539291.06",
            "unit": "ns/iter"
          },
          {
            "name": "citm_catalog_json_large_serde",
            "value": 25934291.7,
            "range": "± 788987.77",
            "unit": "ns/iter"
          },
          {
            "name": "citm_catalog_json_large_value",
            "value": 64696972.3,
            "range": "± 1072695.12",
            "unit": "ns/iter"
          },
          {
            "name": "package_json_ast",
            "value": 62883.13,
            "range": "± 509.49",
            "unit": "ns/iter"
          },
          {
            "name": "package_json_value",
            "value": 83177.23,
            "range": "± 1477.04",
            "unit": "ns/iter"
          },
          {
            "name": "tsconfig_json_ast",
            "value": 22386.81,
            "range": "± 257.55",
            "unit": "ns/iter"
          },
          {
            "name": "tsconfig_json_value",
            "value": 22881.82,
            "range": "± 476.44",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "dsherret@users.noreply.github.com",
            "name": "David Sherret",
            "username": "dsherret"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "8bafd2d4f571c9f1f89a801be863ab78112ccb09",
          "message": "chore: update release script version (#43)",
          "timestamp": "2024-10-19T13:26:33-04:00",
          "tree_id": "0e6f78ebf45b5393997c813fba263aaac50bbd24",
          "url": "https://github.com/dprint/jsonc-parser/commit/8bafd2d4f571c9f1f89a801be863ab78112ccb09"
        },
        "date": 1729358873767,
        "tool": "cargo",
        "benches": [
          {
            "name": "citm_catalog_json_large_ast",
            "value": 46489331.3,
            "range": "± 894378.40",
            "unit": "ns/iter"
          },
          {
            "name": "citm_catalog_json_large_serde",
            "value": 29163009.8,
            "range": "± 2372391.29",
            "unit": "ns/iter"
          },
          {
            "name": "citm_catalog_json_large_value",
            "value": 69601643.6,
            "range": "± 1697083.03",
            "unit": "ns/iter"
          },
          {
            "name": "package_json_ast",
            "value": 62780.22,
            "range": "± 689.59",
            "unit": "ns/iter"
          },
          {
            "name": "package_json_value",
            "value": 82517.58,
            "range": "± 1811.06",
            "unit": "ns/iter"
          },
          {
            "name": "tsconfig_json_ast",
            "value": 22351.95,
            "range": "± 313.21",
            "unit": "ns/iter"
          },
          {
            "name": "tsconfig_json_value",
            "value": 22798.61,
            "range": "± 180.76",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "dprintbot@users.noreply.github.com",
            "name": "dprintbot",
            "username": "dprintbot"
          },
          "committer": {
            "email": "dprintbot@users.noreply.github.com",
            "name": "dprintbot",
            "username": "dprintbot"
          },
          "distinct": true,
          "id": "cd7948863dec71d543c23701582b38446ae1477c",
          "message": "0.24.0",
          "timestamp": "2024-10-19T17:27:04Z",
          "tree_id": "95dacedaaec064998fe59ff2dfacaa46dd19a84c",
          "url": "https://github.com/dprint/jsonc-parser/commit/cd7948863dec71d543c23701582b38446ae1477c"
        },
        "date": 1729358902890,
        "tool": "cargo",
        "benches": [
          {
            "name": "citm_catalog_json_large_ast",
            "value": 44634266.3,
            "range": "± 1994321.46",
            "unit": "ns/iter"
          },
          {
            "name": "citm_catalog_json_large_serde",
            "value": 26858737.6,
            "range": "± 1428849.35",
            "unit": "ns/iter"
          },
          {
            "name": "citm_catalog_json_large_value",
            "value": 70585012,
            "range": "± 3837595.99",
            "unit": "ns/iter"
          },
          {
            "name": "package_json_ast",
            "value": 63081.78,
            "range": "± 804.76",
            "unit": "ns/iter"
          },
          {
            "name": "package_json_value",
            "value": 82954.37,
            "range": "± 1181.27",
            "unit": "ns/iter"
          },
          {
            "name": "tsconfig_json_ast",
            "value": 22289.45,
            "range": "± 1513.81",
            "unit": "ns/iter"
          },
          {
            "name": "tsconfig_json_value",
            "value": 22845.6,
            "range": "± 175.34",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "dsherret@users.noreply.github.com",
            "name": "David Sherret",
            "username": "dsherret"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "d70dd48da96aacedc230c4fa91adc23dc7e78dbf",
          "message": "feat: improve cst manipulation implementation (#44)",
          "timestamp": "2024-10-19T18:01:53-04:00",
          "tree_id": "201336b80e7c5dabea2d65d9bd52fdba9c76dcec",
          "url": "https://github.com/dprint/jsonc-parser/commit/d70dd48da96aacedc230c4fa91adc23dc7e78dbf"
        },
        "date": 1729375394100,
        "tool": "cargo",
        "benches": [
          {
            "name": "citm_catalog_json_large_ast",
            "value": 48330136,
            "range": "± 406219.97",
            "unit": "ns/iter"
          },
          {
            "name": "citm_catalog_json_large_serde",
            "value": 30349279.8,
            "range": "± 1964762.73",
            "unit": "ns/iter"
          },
          {
            "name": "citm_catalog_json_large_value",
            "value": 70916586.8,
            "range": "± 1751065.23",
            "unit": "ns/iter"
          },
          {
            "name": "package_json_ast",
            "value": 63391.77,
            "range": "± 913.85",
            "unit": "ns/iter"
          },
          {
            "name": "package_json_value",
            "value": 82791.22,
            "range": "± 621.60",
            "unit": "ns/iter"
          },
          {
            "name": "tsconfig_json_ast",
            "value": 22374.61,
            "range": "± 295.04",
            "unit": "ns/iter"
          },
          {
            "name": "tsconfig_json_value",
            "value": 22872.78,
            "range": "± 253.92",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "dprintbot@users.noreply.github.com",
            "name": "dprintbot",
            "username": "dprintbot"
          },
          "committer": {
            "email": "dprintbot@users.noreply.github.com",
            "name": "dprintbot",
            "username": "dprintbot"
          },
          "distinct": true,
          "id": "bd193327e2afef28f339148c6d18ef18a80e7089",
          "message": "0.25.0",
          "timestamp": "2024-10-19T22:02:30Z",
          "tree_id": "a7d25edf429a8b1d4fd0f2369ecee685eb11f1a6",
          "url": "https://github.com/dprint/jsonc-parser/commit/bd193327e2afef28f339148c6d18ef18a80e7089"
        },
        "date": 1729375429294,
        "tool": "cargo",
        "benches": [
          {
            "name": "citm_catalog_json_large_ast",
            "value": 45679128.1,
            "range": "± 1075297.41",
            "unit": "ns/iter"
          },
          {
            "name": "citm_catalog_json_large_serde",
            "value": 27396562.8,
            "range": "± 1787129.19",
            "unit": "ns/iter"
          },
          {
            "name": "citm_catalog_json_large_value",
            "value": 66734716.7,
            "range": "± 2557206.27",
            "unit": "ns/iter"
          },
          {
            "name": "package_json_ast",
            "value": 63945.98,
            "range": "± 716.35",
            "unit": "ns/iter"
          },
          {
            "name": "package_json_value",
            "value": 83213.31,
            "range": "± 1842.94",
            "unit": "ns/iter"
          },
          {
            "name": "tsconfig_json_ast",
            "value": 22513.61,
            "range": "± 324.02",
            "unit": "ns/iter"
          },
          {
            "name": "tsconfig_json_value",
            "value": 23050.51,
            "range": "± 1168.74",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "dsherret@users.noreply.github.com",
            "name": "David Sherret",
            "username": "dsherret"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "8ab43d24340c30926502c0134628cb8024ca73d8",
          "message": "feat(cst): add `_or_set` methods (#45)",
          "timestamp": "2024-10-20T00:35:46-04:00",
          "tree_id": "11f46d48babbf606ecb8554af0a5e5a9bb4cc226",
          "url": "https://github.com/dprint/jsonc-parser/commit/8ab43d24340c30926502c0134628cb8024ca73d8"
        },
        "date": 1729399025832,
        "tool": "cargo",
        "benches": [
          {
            "name": "citm_catalog_json_large_ast",
            "value": 46691448.3,
            "range": "± 504366.95",
            "unit": "ns/iter"
          },
          {
            "name": "citm_catalog_json_large_serde",
            "value": 30464398.8,
            "range": "± 3643073.79",
            "unit": "ns/iter"
          },
          {
            "name": "citm_catalog_json_large_value",
            "value": 70826502.3,
            "range": "± 2742860.48",
            "unit": "ns/iter"
          },
          {
            "name": "package_json_ast",
            "value": 62961.53,
            "range": "± 898.91",
            "unit": "ns/iter"
          },
          {
            "name": "package_json_value",
            "value": 83049.6,
            "range": "± 821.40",
            "unit": "ns/iter"
          },
          {
            "name": "tsconfig_json_ast",
            "value": 22347.88,
            "range": "± 271.84",
            "unit": "ns/iter"
          },
          {
            "name": "tsconfig_json_value",
            "value": 22887.57,
            "range": "± 198.04",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "dprintbot@users.noreply.github.com",
            "name": "dprintbot",
            "username": "dprintbot"
          },
          "committer": {
            "email": "dprintbot@users.noreply.github.com",
            "name": "dprintbot",
            "username": "dprintbot"
          },
          "distinct": true,
          "id": "c08992d3adc1a72d9354c209fd3cf3ca70061b24",
          "message": "0.25.1",
          "timestamp": "2024-10-20T04:36:19Z",
          "tree_id": "f289ce7c9b98e8f9505703583881bc12c6f72e8b",
          "url": "https://github.com/dprint/jsonc-parser/commit/c08992d3adc1a72d9354c209fd3cf3ca70061b24"
        },
        "date": 1729399056739,
        "tool": "cargo",
        "benches": [
          {
            "name": "citm_catalog_json_large_ast",
            "value": 44952256.2,
            "range": "± 1287259.71",
            "unit": "ns/iter"
          },
          {
            "name": "citm_catalog_json_large_serde",
            "value": 26902921.7,
            "range": "± 2263749.03",
            "unit": "ns/iter"
          },
          {
            "name": "citm_catalog_json_large_value",
            "value": 66587792.1,
            "range": "± 2054455.28",
            "unit": "ns/iter"
          },
          {
            "name": "package_json_ast",
            "value": 63381.86,
            "range": "± 469.23",
            "unit": "ns/iter"
          },
          {
            "name": "package_json_value",
            "value": 82572.59,
            "range": "± 1523.32",
            "unit": "ns/iter"
          },
          {
            "name": "tsconfig_json_ast",
            "value": 22304.13,
            "range": "± 192.28",
            "unit": "ns/iter"
          },
          {
            "name": "tsconfig_json_value",
            "value": 22878.65,
            "range": "± 283.20",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "dsherret@users.noreply.github.com",
            "name": "David Sherret",
            "username": "dsherret"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "ea87b57fb5b312e80ea1a9bbc6a545fa9ea6f0c4",
          "message": "feat(cst): CstObject.ensure_multiline() (#46)",
          "timestamp": "2024-10-21T11:00:29-04:00",
          "tree_id": "be4cc36fac6a2bf6675e21e6b6e32d098badd699",
          "url": "https://github.com/dprint/jsonc-parser/commit/ea87b57fb5b312e80ea1a9bbc6a545fa9ea6f0c4"
        },
        "date": 1729522931113,
        "tool": "cargo",
        "benches": [
          {
            "name": "citm_catalog_json_large_ast",
            "value": 46415597.9,
            "range": "± 967936.57",
            "unit": "ns/iter"
          },
          {
            "name": "citm_catalog_json_large_serde",
            "value": 28770825.7,
            "range": "± 2456268.29",
            "unit": "ns/iter"
          },
          {
            "name": "citm_catalog_json_large_value",
            "value": 69610122.6,
            "range": "± 1836632.98",
            "unit": "ns/iter"
          },
          {
            "name": "package_json_ast",
            "value": 63330.61,
            "range": "± 925.68",
            "unit": "ns/iter"
          },
          {
            "name": "package_json_value",
            "value": 82883.46,
            "range": "± 853.16",
            "unit": "ns/iter"
          },
          {
            "name": "tsconfig_json_ast",
            "value": 22302.87,
            "range": "± 232.93",
            "unit": "ns/iter"
          },
          {
            "name": "tsconfig_json_value",
            "value": 22903.47,
            "range": "± 300.02",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "dprintbot@users.noreply.github.com",
            "name": "dprintbot",
            "username": "dprintbot"
          },
          "committer": {
            "email": "dprintbot@users.noreply.github.com",
            "name": "dprintbot",
            "username": "dprintbot"
          },
          "distinct": true,
          "id": "800c08763866518c376cffb6516c6e15a27dbe9c",
          "message": "0.25.2",
          "timestamp": "2024-10-21T15:01:17Z",
          "tree_id": "beef9dec8e7f95466c9cbdf6e9993564dfc991ce",
          "url": "https://github.com/dprint/jsonc-parser/commit/800c08763866518c376cffb6516c6e15a27dbe9c"
        },
        "date": 1729522958993,
        "tool": "cargo",
        "benches": [
          {
            "name": "citm_catalog_json_large_ast",
            "value": 45116640.2,
            "range": "± 764496.18",
            "unit": "ns/iter"
          },
          {
            "name": "citm_catalog_json_large_serde",
            "value": 26580944.4,
            "range": "± 2303272.33",
            "unit": "ns/iter"
          },
          {
            "name": "citm_catalog_json_large_value",
            "value": 65900702.5,
            "range": "± 2014156.39",
            "unit": "ns/iter"
          },
          {
            "name": "package_json_ast",
            "value": 63755.54,
            "range": "± 678.22",
            "unit": "ns/iter"
          },
          {
            "name": "package_json_value",
            "value": 83001.55,
            "range": "± 831.64",
            "unit": "ns/iter"
          },
          {
            "name": "tsconfig_json_ast",
            "value": 22367.98,
            "range": "± 307.06",
            "unit": "ns/iter"
          },
          {
            "name": "tsconfig_json_value",
            "value": 22911.02,
            "range": "± 342.24",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "dsherret@users.noreply.github.com",
            "name": "David Sherret",
            "username": "dsherret"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "c0d54c8bec00b09564236530866708535053e1df",
          "message": "feat(cst): insert into object or array returning get node (#47)",
          "timestamp": "2024-10-21T12:01:01-04:00",
          "tree_id": "c3bd449953c42532e6965c355ba44fc2d46884d3",
          "url": "https://github.com/dprint/jsonc-parser/commit/c0d54c8bec00b09564236530866708535053e1df"
        },
        "date": 1729526544925,
        "tool": "cargo",
        "benches": [
          {
            "name": "citm_catalog_json_large_ast",
            "value": 47056387.9,
            "range": "± 1486166.09",
            "unit": "ns/iter"
          },
          {
            "name": "citm_catalog_json_large_serde",
            "value": 31751972.6,
            "range": "± 1321350.39",
            "unit": "ns/iter"
          },
          {
            "name": "citm_catalog_json_large_value",
            "value": 70429307.9,
            "range": "± 1381141.48",
            "unit": "ns/iter"
          },
          {
            "name": "package_json_ast",
            "value": 63248.19,
            "range": "± 291.49",
            "unit": "ns/iter"
          },
          {
            "name": "package_json_value",
            "value": 82817.46,
            "range": "± 1117.60",
            "unit": "ns/iter"
          },
          {
            "name": "tsconfig_json_ast",
            "value": 22325.01,
            "range": "± 138.96",
            "unit": "ns/iter"
          },
          {
            "name": "tsconfig_json_value",
            "value": 22860.54,
            "range": "± 395.06",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "dprintbot@users.noreply.github.com",
            "name": "dprintbot",
            "username": "dprintbot"
          },
          "committer": {
            "email": "dprintbot@users.noreply.github.com",
            "name": "dprintbot",
            "username": "dprintbot"
          },
          "distinct": true,
          "id": "b42054f86141958db306f5f4239af1912e0d01c3",
          "message": "0.26.0",
          "timestamp": "2024-10-21T16:01:38Z",
          "tree_id": "45ce2cc6433f7ddab0c6c6df7eb8442e88976716",
          "url": "https://github.com/dprint/jsonc-parser/commit/b42054f86141958db306f5f4239af1912e0d01c3"
        },
        "date": 1729526579353,
        "tool": "cargo",
        "benches": [
          {
            "name": "citm_catalog_json_large_ast",
            "value": 45514106.8,
            "range": "± 1323271.41",
            "unit": "ns/iter"
          },
          {
            "name": "citm_catalog_json_large_serde",
            "value": 27160747.1,
            "range": "± 1436990.43",
            "unit": "ns/iter"
          },
          {
            "name": "citm_catalog_json_large_value",
            "value": 67011432.2,
            "range": "± 1820141.71",
            "unit": "ns/iter"
          },
          {
            "name": "package_json_ast",
            "value": 62779.37,
            "range": "± 730.08",
            "unit": "ns/iter"
          },
          {
            "name": "package_json_value",
            "value": 82969.34,
            "range": "± 1082.33",
            "unit": "ns/iter"
          },
          {
            "name": "tsconfig_json_ast",
            "value": 22308.95,
            "range": "± 228.83",
            "unit": "ns/iter"
          },
          {
            "name": "tsconfig_json_value",
            "value": 22847.14,
            "range": "± 164.02",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "dsherret@users.noreply.github.com",
            "name": "David Sherret",
            "username": "dsherret"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "4cdfcf6e9ec3b55a6b09ee9298c45b612a3b55b1",
          "message": "fix(cst): improve insert for single line json object (#48)",
          "timestamp": "2024-10-21T13:06:49-04:00",
          "tree_id": "14ca06c0eff9dd74d970a23cb9f9b4767236e2df",
          "url": "https://github.com/dprint/jsonc-parser/commit/4cdfcf6e9ec3b55a6b09ee9298c45b612a3b55b1"
        },
        "date": 1729530490579,
        "tool": "cargo",
        "benches": [
          {
            "name": "citm_catalog_json_large_ast",
            "value": 46639458.1,
            "range": "± 1156654.23",
            "unit": "ns/iter"
          },
          {
            "name": "citm_catalog_json_large_serde",
            "value": 28771286.1,
            "range": "± 2486649.35",
            "unit": "ns/iter"
          },
          {
            "name": "citm_catalog_json_large_value",
            "value": 70131368.6,
            "range": "± 2717228.52",
            "unit": "ns/iter"
          },
          {
            "name": "package_json_ast",
            "value": 62810.88,
            "range": "± 770.42",
            "unit": "ns/iter"
          },
          {
            "name": "package_json_value",
            "value": 82653.69,
            "range": "± 1317.31",
            "unit": "ns/iter"
          },
          {
            "name": "tsconfig_json_ast",
            "value": 22329.67,
            "range": "± 335.14",
            "unit": "ns/iter"
          },
          {
            "name": "tsconfig_json_value",
            "value": 24034.11,
            "range": "± 290.62",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "dprintbot@users.noreply.github.com",
            "name": "dprintbot",
            "username": "dprintbot"
          },
          "committer": {
            "email": "dprintbot@users.noreply.github.com",
            "name": "dprintbot",
            "username": "dprintbot"
          },
          "distinct": true,
          "id": "937fb2cd87c332c73619883bd8fe445635d09868",
          "message": "0.26.1",
          "timestamp": "2024-10-21T17:07:23Z",
          "tree_id": "c395558ced9fd803e9e0f9c50954c65145b05b4d",
          "url": "https://github.com/dprint/jsonc-parser/commit/937fb2cd87c332c73619883bd8fe445635d09868"
        },
        "date": 1729530522328,
        "tool": "cargo",
        "benches": [
          {
            "name": "citm_catalog_json_large_ast",
            "value": 44457335.4,
            "range": "± 1415457.33",
            "unit": "ns/iter"
          },
          {
            "name": "citm_catalog_json_large_serde",
            "value": 27302690.8,
            "range": "± 3044632.27",
            "unit": "ns/iter"
          },
          {
            "name": "citm_catalog_json_large_value",
            "value": 66864208.3,
            "range": "± 2484860.29",
            "unit": "ns/iter"
          },
          {
            "name": "package_json_ast",
            "value": 62983.93,
            "range": "± 938.87",
            "unit": "ns/iter"
          },
          {
            "name": "package_json_value",
            "value": 82211.21,
            "range": "± 717.09",
            "unit": "ns/iter"
          },
          {
            "name": "tsconfig_json_ast",
            "value": 22322.45,
            "range": "± 98.79",
            "unit": "ns/iter"
          },
          {
            "name": "tsconfig_json_value",
            "value": 22772.19,
            "range": "± 251.90",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "dsherret@users.noreply.github.com",
            "name": "David Sherret",
            "username": "dsherret"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "ecb1496d0305d7864581fc16274533e5f4579b07",
          "message": "fix: do not panic parsing \\r\\n newlines (#49)",
          "timestamp": "2024-10-25T12:38:09-04:00",
          "tree_id": "31526cd2fb98c3d00bc96eb3f24d46c3a8c69a32",
          "url": "https://github.com/dprint/jsonc-parser/commit/ecb1496d0305d7864581fc16274533e5f4579b07"
        },
        "date": 1729874369118,
        "tool": "cargo",
        "benches": [
          {
            "name": "citm_catalog_json_large_ast",
            "value": 45952719.5,
            "range": "± 2093760.76",
            "unit": "ns/iter"
          },
          {
            "name": "citm_catalog_json_large_serde",
            "value": 25259501.4,
            "range": "± 2007707.34",
            "unit": "ns/iter"
          },
          {
            "name": "citm_catalog_json_large_value",
            "value": 66070221.4,
            "range": "± 3556873.37",
            "unit": "ns/iter"
          },
          {
            "name": "package_json_ast",
            "value": 63704.65,
            "range": "± 442.52",
            "unit": "ns/iter"
          },
          {
            "name": "package_json_value",
            "value": 83031.98,
            "range": "± 1245.23",
            "unit": "ns/iter"
          },
          {
            "name": "tsconfig_json_ast",
            "value": 22354.53,
            "range": "± 222.97",
            "unit": "ns/iter"
          },
          {
            "name": "tsconfig_json_value",
            "value": 22775.74,
            "range": "± 177.84",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "dprintbot@users.noreply.github.com",
            "name": "dprintbot",
            "username": "dprintbot"
          },
          "committer": {
            "email": "dprintbot@users.noreply.github.com",
            "name": "dprintbot",
            "username": "dprintbot"
          },
          "distinct": true,
          "id": "7e148265202b712219d020ed2dbd3574e11022d2",
          "message": "0.26.2",
          "timestamp": "2024-10-25T16:38:43Z",
          "tree_id": "9c6f334d8e8fe266e94e3d189bad856711358d8f",
          "url": "https://github.com/dprint/jsonc-parser/commit/7e148265202b712219d020ed2dbd3574e11022d2"
        },
        "date": 1729874410033,
        "tool": "cargo",
        "benches": [
          {
            "name": "citm_catalog_json_large_ast",
            "value": 45576557.5,
            "range": "± 1153545.16",
            "unit": "ns/iter"
          },
          {
            "name": "citm_catalog_json_large_serde",
            "value": 26778590.9,
            "range": "± 2619707.09",
            "unit": "ns/iter"
          },
          {
            "name": "citm_catalog_json_large_value",
            "value": 67105253.8,
            "range": "± 3059945.94",
            "unit": "ns/iter"
          },
          {
            "name": "package_json_ast",
            "value": 64868.24,
            "range": "± 1405.97",
            "unit": "ns/iter"
          },
          {
            "name": "package_json_value",
            "value": 84173.04,
            "range": "± 704.75",
            "unit": "ns/iter"
          },
          {
            "name": "tsconfig_json_ast",
            "value": 22457.05,
            "range": "± 294.80",
            "unit": "ns/iter"
          },
          {
            "name": "tsconfig_json_value",
            "value": 22931.67,
            "range": "± 177.29",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "dsherret@users.noreply.github.com",
            "name": "David Sherret",
            "username": "dsherret"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "5572e9e0ee2cbb96356a9bcd6830f84f566895b5",
          "message": "fix: do not panic on conversion from ast to serde value with invalid number value (#51)",
          "timestamp": "2025-07-22T10:31:25-04:00",
          "tree_id": "94261ee1be4ee620827cf133e9725147ea56c750",
          "url": "https://github.com/dprint/jsonc-parser/commit/5572e9e0ee2cbb96356a9bcd6830f84f566895b5"
        },
        "date": 1753194764474,
        "tool": "cargo",
        "benches": [
          {
            "name": "citm_catalog_json_large_ast",
            "value": 45005070.2,
            "range": "± 982860.32",
            "unit": "ns/iter"
          },
          {
            "name": "citm_catalog_json_large_serde",
            "value": 27948955,
            "range": "± 2562541.21",
            "unit": "ns/iter"
          },
          {
            "name": "citm_catalog_json_large_value",
            "value": 67520603.6,
            "range": "± 1696606.46",
            "unit": "ns/iter"
          },
          {
            "name": "package_json_ast",
            "value": 59851,
            "range": "± 324.04",
            "unit": "ns/iter"
          },
          {
            "name": "package_json_value",
            "value": 79892.15,
            "range": "± 751.88",
            "unit": "ns/iter"
          },
          {
            "name": "tsconfig_json_ast",
            "value": 22088.21,
            "range": "± 116.98",
            "unit": "ns/iter"
          },
          {
            "name": "tsconfig_json_value",
            "value": 22636.03,
            "range": "± 143.78",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "dprintbot@users.noreply.github.com",
            "name": "dprintbot",
            "username": "dprintbot"
          },
          "committer": {
            "email": "dprintbot@users.noreply.github.com",
            "name": "dprintbot",
            "username": "dprintbot"
          },
          "distinct": true,
          "id": "cf46d1089aac3fe39b477e29f552fc208f0dd46a",
          "message": "0.26.3",
          "timestamp": "2025-07-22T14:32:03Z",
          "tree_id": "a2ee5335fd8aaa3d5c13c53fcc682c28527ad2d8",
          "url": "https://github.com/dprint/jsonc-parser/commit/cf46d1089aac3fe39b477e29f552fc208f0dd46a"
        },
        "date": 1753194801983,
        "tool": "cargo",
        "benches": [
          {
            "name": "citm_catalog_json_large_ast",
            "value": 45728371.8,
            "range": "± 943048.17",
            "unit": "ns/iter"
          },
          {
            "name": "citm_catalog_json_large_serde",
            "value": 29400318.9,
            "range": "± 2074606.87",
            "unit": "ns/iter"
          },
          {
            "name": "citm_catalog_json_large_value",
            "value": 69324080.8,
            "range": "± 3021793.87",
            "unit": "ns/iter"
          },
          {
            "name": "package_json_ast",
            "value": 58732.39,
            "range": "± 442.94",
            "unit": "ns/iter"
          },
          {
            "name": "package_json_value",
            "value": 78387.7,
            "range": "± 882.63",
            "unit": "ns/iter"
          },
          {
            "name": "tsconfig_json_ast",
            "value": 21148.54,
            "range": "± 197.20",
            "unit": "ns/iter"
          },
          {
            "name": "tsconfig_json_value",
            "value": 21704.15,
            "range": "± 282.43",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "dsherret@users.noreply.github.com",
            "name": "David Sherret",
            "username": "dsherret"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "a1e440adb758fa06b4481788265ba39dd573866d",
          "message": "chore: update to Rust 1.89 (#53)",
          "timestamp": "2025-10-11T17:28:31-04:00",
          "tree_id": "67c4de5b1dee7b88096861113c1c38415670085b",
          "url": "https://github.com/dprint/jsonc-parser/commit/a1e440adb758fa06b4481788265ba39dd573866d"
        },
        "date": 1760218189349,
        "tool": "cargo",
        "benches": [
          {
            "name": "citm_catalog_json_large_ast",
            "value": 44797363,
            "range": "± 1627394.6",
            "unit": "ns/iter"
          },
          {
            "name": "citm_catalog_json_large_serde",
            "value": 27264613,
            "range": "± 2809606.82",
            "unit": "ns/iter"
          },
          {
            "name": "citm_catalog_json_large_value",
            "value": 67355280.8,
            "range": "± 4477976.11",
            "unit": "ns/iter"
          },
          {
            "name": "package_json_ast",
            "value": 60039.8,
            "range": "± 778.41",
            "unit": "ns/iter"
          },
          {
            "name": "package_json_value",
            "value": 79403.51,
            "range": "± 691.16",
            "unit": "ns/iter"
          },
          {
            "name": "tsconfig_json_ast",
            "value": 19746.73,
            "range": "± 138.78",
            "unit": "ns/iter"
          },
          {
            "name": "tsconfig_json_value",
            "value": 20281.19,
            "range": "± 93.62",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "dsherret@users.noreply.github.com",
            "name": "David Sherret",
            "username": "dsherret"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "8822b914702658ce6b5721544386936962514c0f",
          "message": "docs: document parse_to_value returning None (#54)",
          "timestamp": "2025-10-11T18:07:59-04:00",
          "tree_id": "846691f62f67c047c99e5af23d8ed211b060396e",
          "url": "https://github.com/dprint/jsonc-parser/commit/8822b914702658ce6b5721544386936962514c0f"
        },
        "date": 1760220551858,
        "tool": "cargo",
        "benches": [
          {
            "name": "citm_catalog_json_large_ast",
            "value": 44640449,
            "range": "± 1138696.81",
            "unit": "ns/iter"
          },
          {
            "name": "citm_catalog_json_large_serde",
            "value": 26799640.5,
            "range": "± 1671095.7",
            "unit": "ns/iter"
          },
          {
            "name": "citm_catalog_json_large_value",
            "value": 63551280.5,
            "range": "± 2412616.24",
            "unit": "ns/iter"
          },
          {
            "name": "package_json_ast",
            "value": 60042.68,
            "range": "± 418.5",
            "unit": "ns/iter"
          },
          {
            "name": "package_json_value",
            "value": 79547.6,
            "range": "± 715.7",
            "unit": "ns/iter"
          },
          {
            "name": "tsconfig_json_ast",
            "value": 21148.06,
            "range": "± 171.67",
            "unit": "ns/iter"
          },
          {
            "name": "tsconfig_json_value",
            "value": 21692.36,
            "range": "± 214.31",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "dsherret@users.noreply.github.com",
            "name": "David Sherret",
            "username": "dsherret"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "3aacc5b89210e42b3e22f54d98f58901534e87a6",
          "message": "docs: fix output for cst example (#55)",
          "timestamp": "2025-10-11T19:33:45-04:00",
          "tree_id": "97d68fd6ae5e1d6af92801e0cc96ce45239c0f3c",
          "url": "https://github.com/dprint/jsonc-parser/commit/3aacc5b89210e42b3e22f54d98f58901534e87a6"
        },
        "date": 1760225698244,
        "tool": "cargo",
        "benches": [
          {
            "name": "citm_catalog_json_large_ast",
            "value": 44999977.9,
            "range": "± 1028839.39",
            "unit": "ns/iter"
          },
          {
            "name": "citm_catalog_json_large_serde",
            "value": 28451533.7,
            "range": "± 3103353.07",
            "unit": "ns/iter"
          },
          {
            "name": "citm_catalog_json_large_value",
            "value": 68398367.2,
            "range": "± 1946619.98",
            "unit": "ns/iter"
          },
          {
            "name": "package_json_ast",
            "value": 66357.78,
            "range": "± 1289.38",
            "unit": "ns/iter"
          },
          {
            "name": "package_json_value",
            "value": 82029.79,
            "range": "± 1236.28",
            "unit": "ns/iter"
          },
          {
            "name": "tsconfig_json_ast",
            "value": 20016.36,
            "range": "± 205.91",
            "unit": "ns/iter"
          },
          {
            "name": "tsconfig_json_value",
            "value": 20226.84,
            "range": "± 268.6",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "dsherret@users.noreply.github.com",
            "name": "David Sherret",
            "username": "dsherret"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "095cd2a49237ddc0f8a04448f9b97b1b18e11557",
          "message": "feat(cst): add `to_serde_value()` for converting a CST node to a `serde_json::Value` (#56)",
          "timestamp": "2025-10-19T16:42:59-04:00",
          "tree_id": "7b2e4e6be8816be17166c867c58e5799b1f6ac39",
          "url": "https://github.com/dprint/jsonc-parser/commit/095cd2a49237ddc0f8a04448f9b97b1b18e11557"
        },
        "date": 1760906659109,
        "tool": "cargo",
        "benches": [
          {
            "name": "citm_catalog_json_large_ast",
            "value": 44107073.6,
            "range": "± 2379190.04",
            "unit": "ns/iter"
          },
          {
            "name": "citm_catalog_json_large_serde",
            "value": 27664179.7,
            "range": "± 1938869.63",
            "unit": "ns/iter"
          },
          {
            "name": "citm_catalog_json_large_value",
            "value": 67419839.2,
            "range": "± 1832121.98",
            "unit": "ns/iter"
          },
          {
            "name": "package_json_ast",
            "value": 62508.01,
            "range": "± 584.92",
            "unit": "ns/iter"
          },
          {
            "name": "package_json_value",
            "value": 80991,
            "range": "± 709.98",
            "unit": "ns/iter"
          },
          {
            "name": "tsconfig_json_ast",
            "value": 21440.41,
            "range": "± 338.79",
            "unit": "ns/iter"
          },
          {
            "name": "tsconfig_json_value",
            "value": 21932.23,
            "range": "± 213.5",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "dsherret@users.noreply.github.com",
            "name": "David Sherret",
            "username": "dsherret"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "6c4221d1258313ba916969826e8ea877f4ce29f8",
          "message": "fix: support hexadecimal numbers and words with underscores (#57)",
          "timestamp": "2025-10-19T17:03:08-04:00",
          "tree_id": "b37089523376183f6db6f47273c86f8bf2830848",
          "url": "https://github.com/dprint/jsonc-parser/commit/6c4221d1258313ba916969826e8ea877f4ce29f8"
        },
        "date": 1760907877039,
        "tool": "cargo",
        "benches": [
          {
            "name": "citm_catalog_json_large_ast",
            "value": 46143833.8,
            "range": "± 1988357.28",
            "unit": "ns/iter"
          },
          {
            "name": "citm_catalog_json_large_serde",
            "value": 29148038.8,
            "range": "± 3350797.38",
            "unit": "ns/iter"
          },
          {
            "name": "citm_catalog_json_large_value",
            "value": 68876217.1,
            "range": "± 3345395.36",
            "unit": "ns/iter"
          },
          {
            "name": "package_json_ast",
            "value": 61899.96,
            "range": "± 629.61",
            "unit": "ns/iter"
          },
          {
            "name": "package_json_value",
            "value": 80658.65,
            "range": "± 1098.49",
            "unit": "ns/iter"
          },
          {
            "name": "tsconfig_json_ast",
            "value": 19664.36,
            "range": "± 1500.14",
            "unit": "ns/iter"
          },
          {
            "name": "tsconfig_json_value",
            "value": 21676.71,
            "range": "± 176.44",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "dsherret@users.noreply.github.com",
            "name": "David Sherret",
            "username": "dsherret"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "b35344b35c6d06edb4fdecf217453754d6b05dc1",
          "message": "fix: support numbers with plus sign at start (#58)",
          "timestamp": "2025-10-19T17:14:20-04:00",
          "tree_id": "ab67e90ae87fceabdd2609244c5e0c7366fbe63d",
          "url": "https://github.com/dprint/jsonc-parser/commit/b35344b35c6d06edb4fdecf217453754d6b05dc1"
        },
        "date": 1760908545181,
        "tool": "cargo",
        "benches": [
          {
            "name": "citm_catalog_json_large_ast",
            "value": 47038358.1,
            "range": "± 1736784.04",
            "unit": "ns/iter"
          },
          {
            "name": "citm_catalog_json_large_serde",
            "value": 28186715.1,
            "range": "± 1457301.6",
            "unit": "ns/iter"
          },
          {
            "name": "citm_catalog_json_large_value",
            "value": 68867343.8,
            "range": "± 2338770.27",
            "unit": "ns/iter"
          },
          {
            "name": "package_json_ast",
            "value": 64537.09,
            "range": "± 852.1",
            "unit": "ns/iter"
          },
          {
            "name": "package_json_value",
            "value": 83184.72,
            "range": "± 1569.3",
            "unit": "ns/iter"
          },
          {
            "name": "tsconfig_json_ast",
            "value": 22091.35,
            "range": "± 228.73",
            "unit": "ns/iter"
          },
          {
            "name": "tsconfig_json_value",
            "value": 22585.67,
            "range": "± 334.75",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "dprintbot@users.noreply.github.com",
            "name": "dprintbot",
            "username": "dprintbot"
          },
          "committer": {
            "email": "dprintbot@users.noreply.github.com",
            "name": "dprintbot",
            "username": "dprintbot"
          },
          "distinct": true,
          "id": "52b0ca71186359356de2864dc5987c7509d91401",
          "message": "0.27.0",
          "timestamp": "2025-10-19T21:15:16Z",
          "tree_id": "63cf72e17196e44ed2e4475858af7a9cdc4a7bca",
          "url": "https://github.com/dprint/jsonc-parser/commit/52b0ca71186359356de2864dc5987c7509d91401"
        },
        "date": 1760908600661,
        "tool": "cargo",
        "benches": [
          {
            "name": "citm_catalog_json_large_ast",
            "value": 46649544.8,
            "range": "± 1569791.67",
            "unit": "ns/iter"
          },
          {
            "name": "citm_catalog_json_large_serde",
            "value": 28350625.2,
            "range": "± 2754555",
            "unit": "ns/iter"
          },
          {
            "name": "citm_catalog_json_large_value",
            "value": 69632477.6,
            "range": "± 3243528.27",
            "unit": "ns/iter"
          },
          {
            "name": "package_json_ast",
            "value": 65955.11,
            "range": "± 721.48",
            "unit": "ns/iter"
          },
          {
            "name": "package_json_value",
            "value": 84772.08,
            "range": "± 840.81",
            "unit": "ns/iter"
          },
          {
            "name": "tsconfig_json_ast",
            "value": 22169.88,
            "range": "± 204.27",
            "unit": "ns/iter"
          },
          {
            "name": "tsconfig_json_value",
            "value": 22630.14,
            "range": "± 282.47",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "marcoconcettorudilosso@gmail.com",
            "name": "Marco Concetto Rudilosso",
            "username": "Maaarcocr"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "d9e943083fd244e3a546f7fa0544737fefa64911",
          "message": "fix: support UTF-16 surrogate pairs in unicode escape sequences (#59)",
          "timestamp": "2025-11-12T10:27:37-05:00",
          "tree_id": "b35301ca98f70eeaa254daed388fce6ef77d0d3c",
          "url": "https://github.com/dprint/jsonc-parser/commit/d9e943083fd244e3a546f7fa0544737fefa64911"
        },
        "date": 1762961337632,
        "tool": "cargo",
        "benches": [
          {
            "name": "citm_catalog_json_large_ast",
            "value": 45726170.8,
            "range": "± 640910.55",
            "unit": "ns/iter"
          },
          {
            "name": "citm_catalog_json_large_serde",
            "value": 31033960,
            "range": "± 2891167.43",
            "unit": "ns/iter"
          },
          {
            "name": "citm_catalog_json_large_value",
            "value": 69261873.8,
            "range": "± 1193238.49",
            "unit": "ns/iter"
          },
          {
            "name": "package_json_ast",
            "value": 63711.25,
            "range": "± 765.42",
            "unit": "ns/iter"
          },
          {
            "name": "package_json_value",
            "value": 82808.22,
            "range": "± 1138.08",
            "unit": "ns/iter"
          },
          {
            "name": "tsconfig_json_ast",
            "value": 22073.21,
            "range": "± 243.99",
            "unit": "ns/iter"
          },
          {
            "name": "tsconfig_json_value",
            "value": 22610.01,
            "range": "± 160.03",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "dsherret@users.noreply.github.com",
            "name": "David Sherret",
            "username": "dsherret"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "83e255339675e7f6974236e97e3b98bc62e2af13",
          "message": "refactor: extract parse_hex_char function (#60)",
          "timestamp": "2025-11-12T10:58:59-05:00",
          "tree_id": "df941875ff8b0b979503b52f76bcd445ca6721d1",
          "url": "https://github.com/dprint/jsonc-parser/commit/83e255339675e7f6974236e97e3b98bc62e2af13"
        },
        "date": 1762963216079,
        "tool": "cargo",
        "benches": [
          {
            "name": "citm_catalog_json_large_ast",
            "value": 44903057.8,
            "range": "± 2006797.05",
            "unit": "ns/iter"
          },
          {
            "name": "citm_catalog_json_large_serde",
            "value": 27890576.9,
            "range": "± 2846947.19",
            "unit": "ns/iter"
          },
          {
            "name": "citm_catalog_json_large_value",
            "value": 65880656.2,
            "range": "± 4109816.66",
            "unit": "ns/iter"
          },
          {
            "name": "package_json_ast",
            "value": 62245.48,
            "range": "± 776.36",
            "unit": "ns/iter"
          },
          {
            "name": "package_json_value",
            "value": 83111.14,
            "range": "± 940.08",
            "unit": "ns/iter"
          },
          {
            "name": "tsconfig_json_ast",
            "value": 21924.63,
            "range": "± 192.05",
            "unit": "ns/iter"
          },
          {
            "name": "tsconfig_json_value",
            "value": 22426.29,
            "range": "± 195.86",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "dprintbot@users.noreply.github.com",
            "name": "dprintbot",
            "username": "dprintbot"
          },
          "committer": {
            "email": "dprintbot@users.noreply.github.com",
            "name": "dprintbot",
            "username": "dprintbot"
          },
          "distinct": true,
          "id": "1d74daf5e09f6d8ed863739de9fab2dbe30dc563",
          "message": "0.27.1",
          "timestamp": "2025-11-12T16:00:05Z",
          "tree_id": "313b0aa9f1aa052f46a4dc1ea384a9ca8469ca47",
          "url": "https://github.com/dprint/jsonc-parser/commit/1d74daf5e09f6d8ed863739de9fab2dbe30dc563"
        },
        "date": 1762963279936,
        "tool": "cargo",
        "benches": [
          {
            "name": "citm_catalog_json_large_ast",
            "value": 45200790,
            "range": "± 1507079.71",
            "unit": "ns/iter"
          },
          {
            "name": "citm_catalog_json_large_serde",
            "value": 27759867.5,
            "range": "± 2616540.62",
            "unit": "ns/iter"
          },
          {
            "name": "citm_catalog_json_large_value",
            "value": 67596830.9,
            "range": "± 5154383.94",
            "unit": "ns/iter"
          },
          {
            "name": "package_json_ast",
            "value": 63730.74,
            "range": "± 971.19",
            "unit": "ns/iter"
          },
          {
            "name": "package_json_value",
            "value": 84535.28,
            "range": "± 982.06",
            "unit": "ns/iter"
          },
          {
            "name": "tsconfig_json_ast",
            "value": 21979.99,
            "range": "± 330.77",
            "unit": "ns/iter"
          },
          {
            "name": "tsconfig_json_value",
            "value": 22612.6,
            "range": "± 256.96",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "dsherret@users.noreply.github.com",
            "name": "David Sherret",
            "username": "dsherret"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "d9bf04edbb1edd13af63d9c6750e06ddce33e1c6",
          "message": "feat: add more conditional parse options (#61)",
          "timestamp": "2025-11-26T22:04:34-05:00",
          "tree_id": "67fc5230288ab04f3a00240403481b2a556de89b",
          "url": "https://github.com/dprint/jsonc-parser/commit/d9bf04edbb1edd13af63d9c6750e06ddce33e1c6"
        },
        "date": 1764212747797,
        "tool": "cargo",
        "benches": [
          {
            "name": "citm_catalog_json_large_ast",
            "value": 44932177.4,
            "range": "± 1273552.28",
            "unit": "ns/iter"
          },
          {
            "name": "citm_catalog_json_large_serde",
            "value": 25925206.4,
            "range": "± 1982983.23",
            "unit": "ns/iter"
          },
          {
            "name": "citm_catalog_json_large_value",
            "value": 69697529.1,
            "range": "± 3324769.78",
            "unit": "ns/iter"
          },
          {
            "name": "package_json_ast",
            "value": 60452.96,
            "range": "± 673.91",
            "unit": "ns/iter"
          },
          {
            "name": "package_json_value",
            "value": 80441.91,
            "range": "± 1125.67",
            "unit": "ns/iter"
          },
          {
            "name": "tsconfig_json_ast",
            "value": 19661.3,
            "range": "± 100.34",
            "unit": "ns/iter"
          },
          {
            "name": "tsconfig_json_value",
            "value": 20219.1,
            "range": "± 182.42",
            "unit": "ns/iter"
          }
        ]
      }
    ]
  }
}