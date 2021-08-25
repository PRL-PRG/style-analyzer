# Model report for file:///tmp/top-repos-quality-repos-mewp_bos/tecnologie-per-iot.git HEAD 17a93fceceefa94f249d80f047eaeabed377410d

### Dump

```json
{'created_at': '2021-08-22 16:59:22',
 'datasets': [],
 'dependencies': [],
 'description': 'Model bound to style.format.analyzer.FormatAnalyzer Lookout analyzer.',
 'environment': {'packages': 'ConfigArgParse==0.13.0 Jinja2==2.10 MarkupSafe==1.1.1 PyStemmer==1.3.0 PyYAML==5.1 Pympler==0.5 SQLAlchemy==1.2.10 SQLAlchemy-Utils==0.33.3 asdf==2.3.2 bblfsh==2.12.7 boto==2.49.0 boto3==1.9.130 botocore==1.12.130 cachetools==2.0.1 certifi==2019.3.9 chardet==3.0.4 clint==0.5.1 docker==3.7.0 docker-pycreds==0.4.0 dulwich==0.19.11 grpcio==1.19.0 grpcio-tools==1.19.0 humanfriendly==4.16.1 humanize==0.5.1 idna==2.8 jmespath==0.9.4 jsonschema==2.6.0 lookout-sdk==0.4.1 lookout-sdk-ml==0.19.0 lookout-style==0.2.0 lz4==2.1.6 modelforge==0.12.1 numpy==1.16.2 packaging==19.0 pandas==0.22.0 pip==19.0.3 protobuf==3.7.0 psycopg2-binary==2.7.5 pygtrie==2.3 pyparsing==2.3.1 python-dateutil==2.8.0 python-igraph==0.7.1.post6 pytz==2019.1 requests==2.21.0 requirements-parser==0.2.0 scikit-learn==0.20.1 scikit-optimize==0.5.2 scipy==1.2.1 semantic-version==2.6.0 setuptools==40.8.0 six==1.12.0 smart-open==1.8.1 sourced-ml==0.8.2 spdx==2.5.0 stringcase==1.2.0 tabulate==0.8.2 tqdm==4.31.1 '
                             'urllib3==1.24.1 websocket-client==0.55.0 xxhash==1.3.0',
                 'platform': 'Linux-5.4.0-81-generic-x86_64-with-Ubuntu-18.04-bionic',
                 'python': '3.6.7 (default, Oct 22 2018, 11:32:17) [GCC 8.2.0]'},
 'license': 'ODbL-1.0',
 'metrics': {},
 'model': 'style.format.analyzer.FormatAnalyzer',
 'references': [],
 'series': 'Lookout',
 'size': '15.9 kB',
 'tags': [],
 'uuid': '337d7bab-1919-4c86-9cb1-aad3dd383cdc',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-mewp_bos/tecnologie-per-iot.git 17a93fceceefa94f249d80f047eaeabed377410d

# javascript
53 rules, avg.len. 5.5
## train
PPCR: 0.725914
### report
macro
{'f1-score': 0.39347466972283424,
 'precision': 0.4008765382295217,
 'recall': 0.3875528656178899,
 'support': 10345}
micro
{'f1-score': 0.9253745770903818,
 'precision': 0.9253745770903818,
 'recall': 0.9253745770903818,
 'support': 10345}
weighted
{'f1-score': 0.9211775869488777,
 'precision': 0.917613017061563,
 'recall': 0.9253745770903818,
 'support': 10345}
### report_full
macro
{'f1-score': 0.33015189609075224,
 'precision': 0.4008765382295217,
 'recall': 0.2879363101440096,
 'support': 14251}
micro
{'f1-score': 0.7784192551634412,
 'precision': 0.9253745770903818,
 'recall': 0.6717423338713073,
 'support': 14251}
weighted
{'f1-score': 0.7257485698584935,
 'precision': 0.7989071137977668,
 'recall': 0.6717423338713073,
 'support': 14251}
## test
PPCR: 0.679665
### report
macro
{'f1-score': 0.40822520663788386,
 'precision': 0.4115280289330922,
 'recall': 0.40518030687357304,
 'support': 2841}
micro
{'f1-score': 0.9535374868004224,
 'precision': 0.9535374868004224,
 'recall': 0.9535374868004224,
 'support': 2841}
weighted
{'f1-score': 0.9501111223258172,
 'precision': 0.9471482229024368,
 'recall': 0.9535374868004224,
 'support': 2841}
### report_full
macro
{'f1-score': 0.3254279250956991,
 'precision': 0.4115280289330922,
 'recall': 0.2774973648358699,
 'support': 4180}
micro
{'f1-score': 0.7716849451645065,
 'precision': 0.9535374868004224,
 'recall': 0.6480861244019138,
 'support': 4180}
weighted
{'f1-score': 0.722531061576349,
 'precision': 0.8478562776330931,
 'recall': 0.6480861244019138,
 'support': 4180}
```

## javascript
### Summary
35 rules, avg.len. 5.3

| | |
|-|-|
|Min support|159|
|Max support|2131|
|Min confidence|0.9350152611732483|
|Max confidence|0.9997653961181641|

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
                     'min_samples_leaf': 105,
                     'min_samples_split': 180,
                     'n_estimators': 10,
                     'prune_attributes': True,
                     'prune_branches_algorithms': ['reduced-error'],
                     'prune_dataset_ratio': 0.2,
                     'top_down_greedy_budget': [False, 0.5]}}
```

### Rules

| rule number | description |
|----:|:-----|
| 1 | `  ^1.roles in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 1.000. Support: 2131.` |
| 2 | `  -1.roles in {STRING}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = "<br>Confidence: 0.999. Support: 378.` |
| 3 | `  -1.roles in {EXPRESSION} and not in {STRING}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>	∧ ^2.roles not in {VARIABLE}<br>⇒ y = ∅<br>Confidence: 0.965. Support: 1631.` |
| 4 | `  -1.reserved = (<br>	∧ -1.roles not in {EXPRESSION, STRING}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.936. Support: 666.` |
| 5 | `  -1.reserved not in {(}<br>	∧ -1.roles not in {EXPRESSION, STRING}<br>	∧ +1.reserved = ;<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 443.` |
| 6 | `  ^1.internal_type = MemberExpression<br>⇒ y = ∅<br>Confidence: 0.999. Support: 2082.` |
| 7 | `  -1.internal_type = Identifier<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.roles not in {VARIABLE}<br>⇒ y = ∅<br>Confidence: 0.974. Support: 1368.` |
| 8 | `  -1.internal_type not in {Identifier}<br>	∧ -1.reserved = (<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.935. Support: 731.` |
| 9 | `  -1.internal_type not in {Identifier}<br>	∧ -1.reserved not in {(}<br>	∧ +1.reserved = ;<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.967. Support: 599.` |
| 10 | `  -1.internal_type = StringLiteral<br>	∧ -1.reserved not in {(}<br>	∧ +1.reserved not in {;}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = "<br>Confidence: 0.998. Support: 252.` |
| 11 | `  -1.internal_type not in {Identifier, StringLiteral}<br>	∧ -1.reserved not in {(, ;}<br>	∧ +1.reserved = )<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.949. Support: 264.` |
| 12 | `  •••start_col ≥ 6<br>	∧ -1.reserved not in {(, ;}<br>	∧ -1.roles not in {EXPRESSION, STRING}<br>	∧ +1.reserved = )<br>	∧ +1.roles not in {STRING}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.965. Support: 159.` |
| 13 | `  •••start_col ≥ 6<br>	∧ -1.diff_col ≤ 8<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {EXPRESSION, STRING}<br>	∧ -3.diff_offset ≥ 5<br>	∧ +1.reserved not in {), ;}<br>	∧ +1.roles not in {STRING}<br>	∧ ^1.roles in {DECLARATION} and not in {LITERAL, OPERATOR, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.966. Support: 402.` |
| 14 | `  •••start_col ≥ 6<br>	∧ -1.diff_col ≤ 8<br>	∧ -1.reserved = ,<br>	∧ -1.roles not in {EXPRESSION, STRING}<br>	∧ -3.diff_offset ≥ 5<br>	∧ +1.reserved not in {)}<br>	∧ +1.roles not in {STRING}<br>	∧ ^1.roles not in {DECLARATION, LITERAL, OPERATOR, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.962. Support: 169.` |
| 15 | `  -3.roles in {EXPRESSION}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {OPERATOR} and not in {ARITHMETIC}<br>⇒ y = ␣<br>Confidence: 0.936. Support: 664.` |
| 16 | `  ^1.roles in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 2123.` |
| 17 | `  -1.internal_type = StringLiteral<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = "<br>Confidence: 0.999. Support: 368.` |
| 18 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>	∧ ^2.roles not in {VARIABLE}<br>⇒ y = ∅<br>Confidence: 0.967. Support: 1638.` |
| 19 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.939. Support: 714.` |
| 20 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = ;<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.994. Support: 401.` |
| 21 | `  •••start_col ≥ 6<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ;}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = )<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.980. Support: 171.` |
| 22 | `  -1.roles in {IDENTIFIER}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.roles not in {VARIABLE}<br>⇒ y = ∅<br>Confidence: 0.974. Support: 1371.` |
| 23 | `  -1.reserved = (<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.942. Support: 698.` |
| 24 | `  -1.reserved not in {(}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved = ;<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.974. Support: 529.` |
| 25 | `  -1.reserved not in {(}<br>	∧ -1.roles in {STRING} and not in {IDENTIFIER}<br>	∧ +1.reserved not in {;}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = "<br>Confidence: 0.998. Support: 254.` |
| 26 | `  -1.reserved not in {(}<br>	∧ -1.roles not in {IDENTIFIER, STRING}<br>	∧ +1.reserved = )<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.944. Support: 277.` |
| 27 | `  -1.internal_type not in {StringLiteral}<br>	∧ -3.roles in {EXPRESSION}<br>	∧ ^1.roles in {OPERATOR} and not in {ARITHMETIC, IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.935. Support: 654.` |
| 28 | `  -1.internal_type = StringLiteral<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved not in {;}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = "<br>Confidence: 0.998. Support: 271.` |
| 29 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ;}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved = )<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.980. Support: 278.` |
| 30 | `  •••start_col ≥ 6<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved not in {), ;}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {DECLARATION} and not in {FILE, LITERAL, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.944. Support: 384.` |
| 31 | `  -1.internal_type = Identifier<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>	∧ ^2.roles not in {VARIABLE}<br>⇒ y = ∅<br>Confidence: 0.978. Support: 1424.` |
| 32 | `  -1.internal_type = StringLiteral<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = "<br>Confidence: 0.998. Support: 290.` |
| 33 | `  -1.internal_type not in {Identifier, StringLiteral}<br>	∧ -1.reserved = (<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.945. Support: 659.` |
| 34 | `  -1.internal_type not in {Identifier, StringLiteral}<br>	∧ -1.reserved not in {(}<br>	∧ +1.reserved = ;<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.989. Support: 583.` |
| 35 | `  -1.internal_type not in {Identifier, StringLiteral}<br>	∧ -1.reserved not in {(, ;}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = )<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.950. Support: 251.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 5.257142857142857, "max_conf": 0.9997653961181641, "max_support": 2131, "min_conf": 0.9350152611732483, "min_support": 159, "num_rules": 35}}
```
</details>
