window.BENCHMARK_DATA = {
  "lastUpdate": 1617830509319,
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
      }
    ]
  }
}