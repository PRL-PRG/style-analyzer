# Model report for file:///tmp/top-repos-quality-repos-a8rzqgwl/qgis-webappbuilder-plugin.git HEAD ceed00caa091ca875feef624c1cf4a12bb23cda4

### Dump

```json
{'created_at': '2021-08-29 16:41:31',
 'datasets': [],
 'dependencies': [],
 'description': 'Model bound to style.format.analyzer.FormatAnalyzer Lookout analyzer.',
 'environment': {'packages': 'ConfigArgParse==0.13.0 Jinja2==2.10 MarkupSafe==1.1.1 PyStemmer==1.3.0 PyYAML==5.1 Pympler==0.5 SQLAlchemy==1.2.10 SQLAlchemy-Utils==0.33.3 asdf==2.3.2 bblfsh==2.12.7 boto==2.49.0 boto3==1.9.130 botocore==1.12.130 cachetools==2.0.1 certifi==2019.3.9 chardet==3.0.4 clint==0.5.1 docker==3.7.0 docker-pycreds==0.4.0 dulwich==0.19.11 grpcio==1.19.0 grpcio-tools==1.19.0 humanfriendly==4.16.1 humanize==0.5.1 idna==2.8 jmespath==0.9.4 jsonschema==2.6.0 lookout-sdk==0.4.1 lookout-sdk-ml==0.19.0 lookout-style==0.2.0 lz4==2.1.6 modelforge==0.12.1 numpy==1.16.2 packaging==19.0 pandas==0.22.0 pip==19.0.3 protobuf==3.7.0 psycopg2-binary==2.7.5 pygtrie==2.3 pyparsing==2.3.1 python-dateutil==2.8.0 python-igraph==0.7.1.post6 pytz==2019.1 requests==2.21.0 requirements-parser==0.2.0 scikit-learn==0.20.1 scikit-optimize==0.5.2 scipy==1.2.1 semantic-version==2.6.0 setuptools==40.8.0 six==1.12.0 smart-open==1.8.1 sourced-ml==0.8.2 spdx==2.5.0 stringcase==1.2.0 tabulate==0.8.2 tqdm==4.31.1 '
                             'urllib3==1.24.1 websocket-client==0.55.0 xxhash==1.3.0',
                 'platform': 'Linux-5.4.0-74-generic-x86_64-with-Ubuntu-18.04-bionic',
                 'python': '3.6.7 (default, Oct 22 2018, 11:32:17) [GCC 8.2.0]'},
 'license': 'ODbL-1.0',
 'metrics': {},
 'model': 'style.format.analyzer.FormatAnalyzer',
 'references': [],
 'series': 'Lookout',
 'size': '17.2 kB',
 'tags': [],
 'uuid': '3436bd8d-adb2-4b07-abb5-804582a7c4f1',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-a8rzqgwl/qgis-webappbuilder-plugin.git ceed00caa091ca875feef624c1cf4a12bb23cda4

# javascript
68 rules, avg.len. 4.6
## train
PPCR: 0.984087
### report
macro
{'f1-score': 0.7933167978551674,
 'precision': 0.8099614435737669,
 'recall': 0.7806832055286329,
 'support': 11441}
micro
{'f1-score': 0.9178393497071934,
 'precision': 0.9178393497071934,
 'recall': 0.9178393497071934,
 'support': 11441}
weighted
{'f1-score': 0.9091998702877836,
 'precision': 0.9046775104933157,
 'recall': 0.9178393497071934,
 'support': 11441}
### report_full
macro
{'f1-score': 0.7907324369013816,
 'precision': 0.8099614435737669,
 'recall': 0.7757209073224597,
 'support': 11626}
micro
{'f1-score': 0.9104781722807475,
 'precision': 0.9178393497071934,
 'recall': 0.9032341303973852,
 'support': 11626}
weighted
{'f1-score': 0.8982690731783843,
 'precision': 0.8973927031873415,
 'recall': 0.9032341303973852,
 'support': 11626}
## test
PPCR: 0.982100
### report
macro
{'f1-score': 0.6603376125221219,
 'precision': 0.7624184289059187,
 'recall': 0.6051199817935148,
 'support': 2030}
micro
{'f1-score': 0.8349753694581281,
 'precision': 0.8349753694581281,
 'recall': 0.8349753694581281,
 'support': 2030}
weighted
{'f1-score': 0.8119456866186986,
 'precision': 0.8051908852704135,
 'recall': 0.8349753694581281,
 'support': 2030}
### report_full
macro
{'f1-score': 0.6532181718558461,
 'precision': 0.7624184289059187,
 'recall': 0.5956918892108372,
 'support': 2067}
micro
{'f1-score': 0.8274347083231632,
 'precision': 0.8349753694581281,
 'recall': 0.8200290275761973,
 'support': 2067}
weighted
{'f1-score': 0.800465362628911,
 'precision': 0.7974821411570296,
 'recall': 0.8200290275761973,
 'support': 2067}
```

## javascript
### Summary
50 rules, avg.len. 4.4

| | |
|-|-|
|Min support|154|
|Max support|4626|
|Min confidence|0.9209900498390198|
|Max confidence|0.9987654089927673|

### Configuration

```json
{'feature_extractor': {'cutoff_label_support': 80,
                       'debug_parsing': False,
                       'label_composites': '<cut>',
                       'left_features': ['length',
                                         'diff_offset',
                                         'diff_col',
                                         'diff_line',
                                         'internal_type',
                                         'label',
                                         'reserved',
                                         'roles'],
                       'left_siblings_window': 5,
                       'no_labels_on_right': True,
                       'node_features': ['start_line', 'start_col'],
                       'parent_features': ['internal_type', 'roles'],
                       'parents_depth': 2,
                       'return_sibling_indices': False,
                       'right_features': ['length', 'internal_type', 'reserved', 'roles'],
                       'right_siblings_window': 5,
                       'select_features_number': 500,
                       'selected_features': '<cut>'},
 'line_length_limit': 500,
 'lines_ratio_train_trigger': 0.2,
 'lower_bound_instances': 500,
 'optimizer': {'base_model_name_categories': ['sklearn.ensemble.RandomForestClassifier',
                                              'sklearn.tree.DecisionTreeClassifier'],
               'cv': 3,
               'max_depth_categories': [None, 5, 10],
               'max_features_categories': [None, 'auto'],
               'min_samples_leaf_max': 120,
               'min_samples_leaf_min': 90,
               'min_samples_split_max': 240,
               'min_samples_split_min': 180,
               'n_iter': 50,
               'n_jobs': -1},
 'overall_size_limit': 5242880,
 'random_state': 42,
 'test_dataset_ratio': 0.2,
 'trainable_rules': {'attribute_similarity_threshold': 0.98,
                     'base_model_name': 'sklearn.ensemble.RandomForestClassifier',
                     'confidence_threshold': 0.8,
                     'max_depth': 10,
                     'min_samples_leaf': 90,
                     'min_samples_split': 240,
                     'n_estimators': 10,
                     'prune_attributes': True,
                     'prune_branches_algorithms': ['reduced-error'],
                     'prune_dataset_ratio': 0.2,
                     'top_down_greedy_budget': [False, 0.5]}}
```

### Rules

| rule number | description |
|----:|:-----|
| 1 | `  -1.diff_offset ≥ 3<br>	∧ +1.length ≥ 2<br>⇒ y = ␣<br>Confidence: 0.964. Support: 766.` |
| 2 | `  -1.diff_offset ≤ 2<br>	∧ -1.reserved = {<br>	∧ +1.length ≥ 2<br>	∧ +2.length ≥ 4<br>⇒ y = ⏎␣⁺␣⁺␣⁺␣⁺<br>Confidence: 0.992. Support: 295.` |
| 3 | `  -1.diff_offset ≤ 2<br>	∧ -2.roles in {ARGUMENT}<br>	∧ +1.length ≥ 2<br>	∧ +2.length ≤ 3<br>⇒ y = ␣<br>Confidence: 0.967. Support: 315.` |
| 4 | `  -1.reserved not in {;}<br>	∧ -5.roles in {FUNCTION}<br>	∧ +1.length ≤ 1<br>⇒ y = ␣<br>Confidence: 0.978. Support: 289.` |
| 5 | `  -1.internal_type = StringLiteral<br>	∧ -1.reserved not in {;}<br>	∧ -5.roles not in {FUNCTION}<br>	∧ +1.length ≤ 1<br>⇒ y = "<br>Confidence: 0.997. Support: 196.` |
| 6 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ -5.roles not in {FUNCTION}<br>	∧ +1.reserved = =<br>	∧ +1.length ≤ 1<br>⇒ y = ␣<br>Confidence: 0.926. Support: 195.` |
| 7 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ -5.roles not in {FUNCTION}<br>	∧ +1.reserved not in {=}<br>	∧ +1.length ≤ 1<br>⇒ y = ∅<br>Confidence: 0.921. Support: 4626.` |
| 8 | `  -1.diff_col ≥ 3<br>	∧ +1.length ≥ 2<br>⇒ y = ␣<br>Confidence: 0.964. Support: 800.` |
| 9 | `  -1.diff_col ≤ 2<br>	∧ -1.reserved = {<br>	∧ +1.length ≥ 2<br>	∧ +2.length ≥ 3<br>⇒ y = ⏎␣⁺␣⁺␣⁺␣⁺<br>Confidence: 0.994. Support: 272.` |
| 10 | `  -1.diff_col ≤ 2<br>	∧ -1.reserved not in {{}<br>	∧ -4.reserved = ;<br>	∧ +1.length ≥ 2<br>	∧ +2.length ≥ 3<br>⇒ y = ⏎⏎<br>Confidence: 0.991. Support: 272.` |
| 11 | `  -1.diff_col ≤ 2<br>	∧ -2.roles in {ARGUMENT}<br>	∧ +1.length ≥ 2<br>	∧ +2.length ≤ 2<br>⇒ y = ␣<br>Confidence: 0.980. Support: 328.` |
| 12 | `  -2.roles in {FUNCTION}<br>	∧ +1.reserved not in {}}<br>	∧ +1.length ≤ 1<br>⇒ y = ␣<br>Confidence: 0.959. Support: 330.` |
| 13 | `  -1.internal_type = StringLiteral<br>	∧ -2.roles not in {FUNCTION}<br>	∧ +1.reserved not in {}}<br>	∧ +1.length ≤ 1<br>⇒ y = "<br>Confidence: 0.997. Support: 174.` |
| 14 | `  -1.internal_type not in {StringLiteral}<br>	∧ -2.roles not in {FUNCTION}<br>	∧ +1.reserved = =<br>	∧ +1.length ≤ 1<br>⇒ y = ␣<br>Confidence: 0.951. Support: 174.` |
| 15 | `  -1.internal_type not in {StringLiteral}<br>	∧ -2.roles not in {FUNCTION}<br>	∧ +1.reserved not in {=, }}<br>	∧ +1.length ≤ 1<br>⇒ y = ∅<br>Confidence: 0.924. Support: 4593.` |
| 16 | `  -1.length ≥ 3<br>	∧ +1.length ≥ 2<br>⇒ y = ␣<br>Confidence: 0.963. Support: 795.` |
| 17 | `  -1.reserved = {<br>	∧ -1.length ≤ 2<br>	∧ +1.length ≥ 2<br>	∧ +2.length ≥ 3<br>⇒ y = ⏎␣⁺␣⁺␣⁺␣⁺<br>Confidence: 0.986. Support: 315.` |
| 18 | `  -1.reserved not in {{}<br>	∧ -1.length ≤ 2<br>	∧ -3.label in {<-space>}<br>	∧ +1.length ≥ 2<br>	∧ +2.length ≥ 3<br>⇒ y = ⏎⏎<br>Confidence: 0.998. Support: 304.` |
| 19 | `  -1.length ≤ 2<br>	∧ -2.roles in {ARGUMENT}<br>	∧ +1.length ≥ 2<br>	∧ +2.length ≤ 2<br>⇒ y = ␣<br>Confidence: 0.989. Support: 327.` |
| 20 | `  -1.reserved not in {;}<br>	∧ -2.roles in {FUNCTION}<br>	∧ +1.length ≤ 1<br>⇒ y = ␣<br>Confidence: 0.966. Support: 313.` |
| 21 | `  -1.internal_type = StringLiteral<br>	∧ -1.reserved not in {;}<br>	∧ -2.roles not in {FUNCTION}<br>	∧ +1.length ≤ 1<br>⇒ y = "<br>Confidence: 0.997. Support: 197.` |
| 22 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ -2.roles not in {FUNCTION}<br>	∧ +1.reserved = =<br>	∧ +1.length ≤ 1<br>⇒ y = ␣<br>Confidence: 0.958. Support: 154.` |
| 23 | `  -1.diff_col ≤ 2<br>	∧ -1.reserved = {<br>	∧ +1.length ≥ 2<br>	∧ +2.length ≥ 4<br>⇒ y = ⏎␣⁺␣⁺␣⁺␣⁺<br>Confidence: 0.992. Support: 307.` |
| 24 | `  -1.diff_col ≤ 2<br>	∧ -2.roles in {ARGUMENT}<br>	∧ +1.length ≥ 2<br>	∧ +2.length ≤ 3<br>⇒ y = ␣<br>Confidence: 0.986. Support: 322.` |
| 25 | `  -5.roles in {FUNCTION}<br>	∧ +1.reserved not in {}}<br>	∧ +1.length ≤ 1<br>⇒ y = ␣<br>Confidence: 0.982. Support: 307.` |
| 26 | `  -1.roles in {STRING}<br>	∧ -5.roles not in {FUNCTION}<br>	∧ +1.reserved not in {}}<br>	∧ +1.length ≤ 1<br>⇒ y = "<br>Confidence: 0.997. Support: 172.` |
| 27 | `  -1.reserved not in {{}<br>	∧ -1.length ≤ 2<br>	∧ -4.reserved = ;<br>	∧ +1.length ≥ 2<br>	∧ +2.length ≥ 3<br>⇒ y = ⏎⏎<br>Confidence: 0.998. Support: 254.` |
| 28 | `  -1.internal_type = StringLiteral<br>	∧ -5.roles not in {FUNCTION}<br>	∧ +1.reserved not in {}}<br>	∧ +1.length ≤ 1<br>⇒ y = "<br>Confidence: 0.997. Support: 186.` |
| 29 | `  -1.internal_type not in {StringLiteral}<br>	∧ -5.roles not in {FUNCTION}<br>	∧ +1.reserved = =<br>	∧ +1.length ≤ 1<br>⇒ y = ␣<br>Confidence: 0.957. Support: 173.` |
| 30 | `  -1.internal_type not in {StringLiteral}<br>	∧ -5.roles not in {FUNCTION}<br>	∧ +1.reserved not in {=, }}<br>	∧ +1.length ≤ 1<br>⇒ y = ∅<br>Confidence: 0.926. Support: 4499.` |
| 31 | `  -1.diff_col ≤ 2<br>	∧ -1.reserved not in {{}<br>	∧ -1.roles in {STATEMENT}<br>	∧ +1.length ≥ 2<br>	∧ +2.length ≥ 3<br>⇒ y = ⏎⏎<br>Confidence: 0.998. Support: 287.` |
| 32 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ -2.roles not in {FUNCTION}<br>	∧ +1.reserved not in {=}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.961. Support: 4103.` |
| 33 | `  -1.diff_col ≤ 2<br>	∧ -1.reserved not in {{}<br>	∧ -4.reserved = ;<br>	∧ +1.length ≥ 2<br>	∧ +2.length ≥ 4<br>⇒ y = ⏎⏎<br>Confidence: 0.995. Support: 276.` |
| 34 | `  -1.reserved not in {;}<br>	∧ -1.roles in {STRING}<br>	∧ -2.roles not in {FUNCTION}<br>	∧ +1.length ≤ 1<br>⇒ y = "<br>Confidence: 0.997. Support: 190.` |
| 35 | `  -1.reserved not in {;}<br>	∧ -1.roles not in {STRING}<br>	∧ -2.roles not in {FUNCTION}<br>	∧ +1.reserved = =<br>	∧ +1.length ≤ 1<br>⇒ y = ␣<br>Confidence: 0.936. Support: 180.` |
| 36 | `  -1.reserved not in {;}<br>	∧ -1.roles not in {STRING}<br>	∧ -2.roles not in {FUNCTION}<br>	∧ +1.reserved not in {=}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.959. Support: 4126.` |
| 37 | `  -1.reserved = (<br>	∧ +1.length ≥ 2<br>⇒ y = ∅<br>Confidence: 0.992. Support: 571.` |
| 38 | `  -1.reserved not in {(}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type = MemberExpression<br>⇒ y = ∅<br>Confidence: 0.999. Support: 405.` |
| 39 | `  -1.reserved = {<br>	∧ +1.length ≥ 2<br>	∧ +2.length ≥ 3<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ⏎␣⁺␣⁺␣⁺␣⁺<br>Confidence: 0.990. Support: 335.` |
| 40 | `  -1.reserved not in {(, {}<br>	∧ +1.length ≥ 2<br>	∧ +2.length ≥ 3<br>	∧ +4.roles in {FUNCTION}<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ⏎⏎<br>Confidence: 0.981. Support: 297.` |
| 41 | `  -1.reserved not in {;}<br>	∧ -1.roles not in {STRING}<br>	∧ -2.roles not in {FUNCTION}<br>	∧ +1.reserved not in {=}<br>	∧ +1.length ≤ 1<br>⇒ y = ∅<br>Confidence: 0.933. Support: 4525.` |
| 42 | `  -1.reserved = {<br>	∧ +1.length ≥ 2<br>	∧ +2.length ≥ 3<br>⇒ y = ⏎␣⁺␣⁺␣⁺␣⁺<br>Confidence: 0.990. Support: 341.` |
| 43 | `  -1.reserved not in {{}<br>	∧ -3.label in {<newline>}<br>	∧ +1.length ≥ 2<br>	∧ +2.length ≥ 3<br>⇒ y = ⏎⏎<br>Confidence: 0.977. Support: 326.` |
| 44 | `  -1.reserved = (<br>	∧ +1.length ≥ 2<br>	∧ +2.length ≤ 2<br>⇒ y = ∅<br>Confidence: 0.995. Support: 465.` |
| 45 | `  -1.reserved not in {(}<br>	∧ +1.length ≥ 2<br>	∧ +2.length ≤ 2<br>	∧ ^1.roles in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 385.` |
| 46 | `  -1.reserved not in {(}<br>	∧ -2.diff_offset ≥ 7<br>	∧ +1.roles not in {STRING}<br>	∧ +1.length ≥ 2<br>	∧ +2.length ≤ 2<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.979. Support: 1097.` |
| 47 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ -2.roles not in {FUNCTION}<br>	∧ +1.reserved not in {=}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 2638.` |
| 48 | `  •••start_col ≥ 29<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.roles not in {FUNCTION}<br>	∧ +1.reserved not in {=}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.roles in {STATEMENT}<br>⇒ y = ∅<br>Confidence: 0.930. Support: 265.` |
| 49 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.roles not in {FUNCTION}<br>	∧ +1.reserved not in {=}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.roles not in {STATEMENT}<br>⇒ y = ∅<br>Confidence: 0.938. Support: 1145.` |
| 50 | `  -1.internal_type not in {StringLiteral}<br>	∧ -2.roles not in {FUNCTION}<br>	∧ +1.reserved not in {=, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.957. Support: 4038.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 4.38, "max_conf": 0.9987654089927673, "max_support": 4626, "min_conf": 0.9209900498390198, "min_support": 154, "num_rules": 50}}
```
</details>
