window.BENCHMARK_DATA = {
  "lastUpdate": 1639244255355,
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
      }
    ]
  }
}