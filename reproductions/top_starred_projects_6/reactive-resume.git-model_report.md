# Model report for file:///tmp/top-repos-quality-repos-4_ulr_rn/reactive-resume.git HEAD 99d7d3aad24e8a18b304d1563418581e2e9422dc

### Dump

```json
{'created_at': '2021-08-29 22:16:04',
 'datasets': [],
 'dependencies': [],
 'description': 'Model bound to style.format.analyzer.FormatAnalyzer Lookout analyzer.',
 'environment': {'packages': 'ConfigArgParse==0.13.0 Jinja2==2.10 MarkupSafe==1.1.1 PyStemmer==1.3.0 PyYAML==5.1 Pympler==0.5 SQLAlchemy==1.2.10 SQLAlchemy-Utils==0.33.3 asdf==2.3.2 bblfsh==2.12.7 boto==2.49.0 boto3==1.9.130 botocore==1.12.130 cachetools==2.0.1 certifi==2019.3.9 chardet==3.0.4 clint==0.5.1 docker==3.7.0 docker-pycreds==0.4.0 dulwich==0.19.11 grpcio==1.19.0 grpcio-tools==1.19.0 humanfriendly==4.16.1 humanize==0.5.1 idna==2.8 jmespath==0.9.4 jsonschema==2.6.0 lookout-sdk==0.4.1 lookout-sdk-ml==0.19.0 lookout-style==0.2.0 lz4==2.1.6 modelforge==0.12.1 numpy==1.16.2 packaging==19.0 pandas==0.22.0 pip==19.0.3 protobuf==3.7.0 psycopg2-binary==2.7.5 pygtrie==2.3 pyparsing==2.3.1 python-dateutil==2.8.0 python-igraph==0.7.1.post6 pytz==2019.1 requests==2.21.0 requirements-parser==0.2.0 scikit-learn==0.20.1 scikit-optimize==0.5.2 scipy==1.2.1 semantic-version==2.6.0 setuptools==40.8.0 six==1.12.0 smart-open==1.8.1 sourced-ml==0.8.2 spdx==2.5.0 stringcase==1.2.0 tabulate==0.8.2 tqdm==4.31.1 '
                             'urllib3==1.24.1 websocket-client==0.55.0 xxhash==1.3.0',
                 'platform': 'Linux-4.15.0-135-generic-x86_64-with-Ubuntu-18.04-bionic',
                 'python': '3.6.7 (default, Oct 22 2018, 11:32:17) [GCC 8.2.0]'},
 'license': 'ODbL-1.0',
 'metrics': {},
 'model': 'style.format.analyzer.FormatAnalyzer',
 'references': [],
 'series': 'Lookout',
 'size': '18.8 kB',
 'tags': [],
 'uuid': 'ceaedba4-e823-4874-a9b7-9d35461a3ff5',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-4_ulr_rn/reactive-resume.git 99d7d3aad24e8a18b304d1563418581e2e9422dc

# javascript
40 rules, avg.len. 8.3
## train
PPCR: 0.889206
### report
macro
{'f1-score': 0.8006680669252418,
 'precision': 0.840951817554453,
 'recall': 0.77910404600924,
 'support': 41421}
micro
{'f1-score': 0.9565920668260061,
 'precision': 0.9565920668260062,
 'recall': 0.9565920668260062,
 'support': 41421}
weighted
{'f1-score': 0.9539500320370065,
 'precision': 0.9553086282454363,
 'recall': 0.9565920668260062,
 'support': 41421}
### report_full
macro
{'f1-score': 0.71658151627055,
 'precision': 0.840951817554453,
 'recall': 0.6508513335814682,
 'support': 46582}
micro
{'f1-score': 0.9004920286808404,
 'precision': 0.9565920668260062,
 'recall': 0.8506075308058907,
 'support': 46582}
weighted
{'f1-score': 0.891819416704098,
 'precision': 0.9531978234731804,
 'recall': 0.8506075308058907,
 'support': 46582}
## test
PPCR: 0.873150
### report
macro
{'f1-score': 0.7958383193523423,
 'precision': 0.840907866900899,
 'recall': 0.7721337891928957,
 'support': 9912}
micro
{'f1-score': 0.9553066989507667,
 'precision': 0.9553066989507667,
 'recall': 0.9553066989507667,
 'support': 9912}
weighted
{'f1-score': 0.9529591722876618,
 'precision': 0.9547881202123095,
 'recall': 0.9553066989507667,
 'support': 9912}
### report_full
macro
{'f1-score': 0.6899419017532611,
 'precision': 0.840907866900899,
 'recall': 0.6192034090393838,
 'support': 11352}
micro
{'f1-score': 0.8906132430398797,
 'precision': 0.9553066989507667,
 'recall': 0.8341261451726568,
 'support': 11352}
weighted
{'f1-score': 0.8791441333418737,
 'precision': 0.9528593574135474,
 'recall': 0.8341261451726568,
 'support': 11352}
```

## javascript
### Summary
27 rules, avg.len. 7.9

| | |
|-|-|
|Min support|92|
|Max support|14839|
|Min confidence|0.920714259147644|
|Max confidence|0.9992126226425171|

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
                     'base_model_name': 'sklearn.tree.DecisionTreeClassifier',
                     'confidence_threshold': 0.8,
                     'min_samples_leaf': 90,
                     'min_samples_split': 236,
                     'n_estimators': 10,
                     'prune_attributes': True,
                     'prune_branches_algorithms': ['reduced-error'],
                     'prune_dataset_ratio': 0.2,
                     'top_down_greedy_budget': [False, 0.5]}}
```

### Rules

| rule number | description |
|----:|:-----|
| 1 | `  -1.label in {<space>}<br>	∧ ^1.roles in {DECLARATION}<br>⇒ y = '<br>Confidence: 0.999. Support: 635.` |
| 2 | `  -1.label not in {<space>}<br>	∧ +1.reserved = ,<br>	∧ ^1.roles in {DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 288.` |
| 3 | `  -1.label not in {<space>}<br>	∧ +1.reserved = )<br>	∧ ^1.roles in {DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.974. Support: 294.` |
| 4 | `  -1.label not in {<space>}<br>	∧ -3.reserved = )<br>	∧ -5.diff_offset ≤ 6<br>	∧ +1.reserved not in {), ,}<br>	∧ ^1.roles in {DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.981. Support: 185.` |
| 5 | `  -1.label not in {<space>}<br>	∧ -3.reserved = ><br>	∧ +1.reserved not in {,}<br>	∧ ^1.roles in {DECLARATION}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.963. Support: 175.` |
| 6 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ -3.label not in {<newline>}<br>	∧ -3.reserved not in {), >}<br>	∧ +1.reserved not in {), ,}<br>	∧ ^1.roles in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.938. Support: 4303.` |
| 7 | `  -1.internal_type = StringLiteral<br>	∧ -3.roles in {INCOMPLETE}<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = "<br>Confidence: 0.999. Support: 423.` |
| 8 | `  -1.internal_type = StringLiteral<br>	∧ -3.roles not in {INCOMPLETE}<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = '<br>Confidence: 0.998. Support: 1422.` |
| 9 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ;<br>	∧ +1.reserved = }<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.983. Support: 372.` |
| 10 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ;<br>	∧ +1.reserved = import<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ⏎<br>Confidence: 0.956. Support: 511.` |
| 11 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ;<br>	∧ +1.reserved not in {import, }}<br>	∧ ^1.internal_type = Program<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ⏎⏎<br>Confidence: 0.934. Support: 310.` |
| 12 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ -2.internal_type = JSXIdentifier<br>	∧ +1.internal_type = StringLiteral<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = "<br>Confidence: 0.999. Support: 428.` |
| 13 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = :<br>	∧ -2.internal_type not in {JSXIdentifier}<br>	∧ +1.internal_type = StringLiteral<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.993. Support: 209.` |
| 14 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ -3.reserved = ;<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.roles in {EXPRESSION} and not in {DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.930. Support: 352.` |
| 15 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;, }}<br>	∧ -3.reserved = ;<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.roles not in {DECLARATION, EXPRESSION}<br>⇒ y = ␣<br>Confidence: 0.979. Support: 1297.` |
| 16 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +2.reserved = :<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ⏎<br>Confidence: 0.939. Support: 368.` |
| 17 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = :<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.933. Support: 518.` |
| 18 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;}<br>	∧ -3.reserved not in {;}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +2.internal_type not in {JSXIdentifier}<br>	∧ +2.roles in {EXPRESSION}<br>	∧ ^1.roles in {OPERATOR} and not in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.978. Support: 253.` |
| 19 | `  -1.diff_offset ≤ 1<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;}<br>	∧ -3.reserved not in {;}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +2.reserved = =<br>	∧ ^1.internal_type not in {JSXOpeningElement}<br>	∧ ^1.roles not in {DECLARATION, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.978. Support: 200.` |
| 20 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;}<br>	∧ -3.reserved not in {;}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles in {IMPORT}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.945. Support: 263.` |
| 21 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {IMPORT, MAP}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.internal_type = IfStatement<br>	∧ ^1.roles not in {DECLARATION, OPERATOR}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.995. Support: 92.` |
| 22 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :}<br>	∧ -3.reserved = export<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {IMPORT, MAP}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.955. Support: 101.` |
| 23 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;}<br>	∧ -3.label in {<space>}<br>	∧ -3.reserved not in {;}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {IMPORT, MAP}<br>	∧ +2.reserved not in {=}<br>	∧ +2.roles not in {IMPORT}<br>	∧ ^1.roles not in {DECLARATION}<br>	∧ ^2.internal_type = ExpressionStatement<br>⇒ y = ␣<br>Confidence: 0.995. Support: 106.` |
| 24 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;}<br>	∧ -2.internal_type not in {StringLiteral}<br>	∧ -2.label not in {<newline>}<br>	∧ -3.reserved not in {;}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {/}<br>	∧ +1.roles not in {IMPORT, MAP}<br>	∧ +2.reserved = ><br>	∧ +2.roles not in {IMPORT}<br>	∧ ^1.internal_type not in {IfStatement}<br>	∧ ^1.roles not in {DECLARATION, OPERATOR}<br>	∧ ^2.internal_type not in {ExpressionStatement}<br>⇒ y = ∅<br>Confidence: 0.921. Support: 700.` |
| 25 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :}<br>	∧ -2.internal_type not in {StringLiteral}<br>	∧ -2.label not in {<newline>}<br>	∧ -3.reserved = :<br>	∧ -4.label not in {<newline>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {IMPORT, MAP}<br>	∧ +2.reserved not in {=, >}<br>	∧ +2.roles not in {IMPORT}<br>	∧ +2.length ≥ 2<br>	∧ ^1.internal_type not in {IfStatement}<br>	∧ ^1.roles not in {DECLARATION, IF, OPERATOR}<br>	∧ ^2.internal_type not in {ExpressionStatement}<br>⇒ y = ∅<br>Confidence: 0.970. Support: 253.` |
| 26 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ><br>	∧ -2.internal_type not in {StringLiteral}<br>	∧ -2.label not in {<newline>}<br>	∧ -4.label not in {<newline>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {)}<br>	∧ +1.roles not in {IMPORT, MAP}<br>	∧ +2.reserved not in {=, >}<br>	∧ +2.roles not in {IMPORT}<br>	∧ ^1.internal_type not in {IfStatement}<br>	∧ ^1.roles not in {DECLARATION, IF, OPERATOR}<br>	∧ ^2.internal_type not in {ExpressionStatement}<br>⇒ y = ∅<br>Confidence: 0.944. Support: 940.` |
| 27 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, >}<br>	∧ -2.internal_type not in {StringLiteral}<br>	∧ -2.label not in {<newline>}<br>	∧ -3.reserved not in {:, ;}<br>	∧ -4.label not in {<newline>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {IMPORT, MAP}<br>	∧ +2.reserved not in {=, >}<br>	∧ +2.roles not in {IMPORT}<br>	∧ ^1.internal_type not in {IfStatement}<br>	∧ ^1.roles not in {DECLARATION, IF, OPERATOR}<br>	∧ ^2.internal_type not in {ExpressionStatement}<br>⇒ y = ∅<br>Confidence: 0.972. Support: 14839.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 7.888888888888889, "max_conf": 0.9992126226425171, "max_support": 14839, "min_conf": 0.920714259147644, "min_support": 92, "num_rules": 27}}
```
</details>
