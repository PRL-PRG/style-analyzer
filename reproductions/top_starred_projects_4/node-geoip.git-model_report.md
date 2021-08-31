# Model report for file:///tmp/top-repos-quality-repos-ef8spp88/node-geoip.git HEAD 78f4c7f0f3b6fa673035b3cb48c3a2a95be9fb0a

### Dump

```json
{'created_at': '2021-08-30 08:09:39',
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
 'size': '16.5 kB',
 'tags': [],
 'uuid': '4f501e96-024d-4c73-9eaa-c901f93c5ad3',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-ef8spp88/node-geoip.git 78f4c7f0f3b6fa673035b3cb48c3a2a95be9fb0a

# javascript
48 rules, avg.len. 5.5
## train
PPCR: 0.844360
### report
macro
{'f1-score': 0.21641260784338207,
 'precision': 0.21236699401867276,
 'recall': 0.2230342314906365,
 'support': 5539}
micro
{'f1-score': 0.8308358909550461,
 'precision': 0.8308358909550461,
 'recall': 0.8308358909550461,
 'support': 5539}
weighted
{'f1-score': 0.7798934261578434,
 'precision': 0.7405145056487342,
 'recall': 0.8308358909550461,
 'support': 5539}
### report_full
macro
{'f1-score': 0.20691005221935083,
 'precision': 0.21236699401867276,
 'recall': 0.2061288499787715,
 'support': 6560}
micro
{'f1-score': 0.7607240267790727,
 'precision': 0.8308358909550461,
 'recall': 0.7015243902439025,
 'support': 6560}
weighted
{'f1-score': 0.6769364812761681,
 'precision': 0.6651624715557183,
 'recall': 0.7015243902439025,
 'support': 6560}
## test
PPCR: 0.849956
### report
macro
{'f1-score': 0.20167763253359205,
 'precision': 0.20662334109107028,
 'recall': 0.20621685900785897,
 'support': 2906}
micro
{'f1-score': 0.782863041982106,
 'precision': 0.782863041982106,
 'recall': 0.782863041982106,
 'support': 2906}
weighted
{'f1-score': 0.7316686116804189,
 'precision': 0.7134803231325118,
 'recall': 0.782863041982106,
 'support': 2906}
### report_full
macro
{'f1-score': 0.19073669781038788,
 'precision': 0.20662334109107028,
 'recall': 0.1908195873713115,
 'support': 3419}
micro
{'f1-score': 0.7193675889328064,
 'precision': 0.782863041982106,
 'recall': 0.6653992395437263,
 'support': 3419}
weighted
{'f1-score': 0.6392961268667756,
 'precision': 0.6596070810605476,
 'recall': 0.6653992395437263,
 'support': 3419}
```

## javascript
### Summary
32 rules, avg.len. 5.6

| | |
|-|-|
|Min support|142|
|Max support|2217|
|Min confidence|0.9260563254356384|
|Max confidence|0.9988937973976135|

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
                     'min_samples_leaf': 90,
                     'min_samples_split': 182,
                     'n_estimators': 10,
                     'prune_attributes': True,
                     'prune_branches_algorithms': ['reduced-error'],
                     'prune_dataset_ratio': 0.2,
                     'top_down_greedy_budget': [False, 0.5]}}
```

### Rules

| rule number | description |
|----:|:-----|
| 1 | `  ^1.internal_type = MemberExpression<br>⇒ y = ∅<br>Confidence: 0.957. Support: 1004.` |
| 2 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ +1.reserved = =<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ␣<br>Confidence: 0.976. Support: 185.` |
| 3 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.reserved not in {=}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ∅<br>Confidence: 0.926. Support: 1063.` |
| 4 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = ;<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 296.` |
| 5 | `  -1.reserved not in {;}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^2.internal_type = VariableDeclaration<br>⇒ y = ␣<br>Confidence: 0.933. Support: 245.` |
| 6 | `  -1.reserved not in {;}<br>	∧ -1.roles in {IDENTIFIER}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.996. Support: 699.` |
| 7 | `  -1.diff_col ≤ 4<br>	∧ -1.reserved = (<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ -2.label not in {<newline>, <space>}<br>	∧ +1.roles not in {STRING}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 360.` |
| 8 | `  -1.diff_col ≤ 4<br>	∧ -1.reserved not in {(, ,, ;}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ -2.label not in {<newline>, <space>}<br>	∧ +1.reserved = ;<br>	∧ +1.roles not in {STRING}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 263.` |
| 9 | `  ^1.roles in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.956. Support: 970.` |
| 10 | `  -1.reserved not in {;}<br>	∧ ^1.internal_type = VariableDeclarator<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.947. Support: 238.` |
| 11 | `  -1.reserved not in {;}<br>	∧ +1.reserved = =<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.986. Support: 179.` |
| 12 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ +1.reserved = {<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.926. Support: 142.` |
| 13 | `  -1.reserved not in {;}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type = VariableDeclaration<br>⇒ y = ␣<br>Confidence: 0.944. Support: 224.` |
| 14 | `  -1.reserved = )<br>	∧ -1.roles not in {STRING}<br>	∧ -3.diff_line = 0<br>	∧ -3.reserved not in {)}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = ;<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 269.` |
| 15 | `  -1.reserved not in {), ,, ;}<br>	∧ -1.roles not in {STRING}<br>	∧ -3.diff_line = 0<br>	∧ -3.reserved not in {)}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.953. Support: 2217.` |
| 16 | `  -1.reserved not in {;}<br>	∧ -1.roles not in {STRING}<br>	∧ +1.reserved = =<br>	∧ +1.roles not in {STRING}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 174.` |
| 17 | `  -1.reserved not in {;}<br>	∧ -1.roles in {EXPRESSION} and not in {STRING}<br>	∧ +1.reserved not in {=}<br>	∧ +1.roles not in {STRING}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.932. Support: 1073.` |
| 18 | `  -1.reserved = (<br>	∧ -1.roles not in {EXPRESSION, STRING}<br>	∧ +1.reserved not in {=}<br>	∧ +1.roles not in {STRING}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 452.` |
| 19 | `  -1.reserved not in {(, ;}<br>	∧ -1.roles not in {EXPRESSION, STRING}<br>	∧ +1.reserved = ;<br>	∧ +1.roles not in {STRING}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 300.` |
| 20 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.reserved not in {=}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.995. Support: 860.` |
| 21 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = ;<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 278.` |
| 22 | `  ^1.roles in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.952. Support: 1025.` |
| 23 | `  -1.reserved not in {;}<br>	∧ +1.reserved = =<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 182.` |
| 24 | `  -1.reserved not in {;}<br>	∧ -1.roles in {EXPRESSION} and not in {STRING}<br>	∧ +1.reserved not in {=}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.960. Support: 983.` |
| 25 | `  -1.reserved not in {;}<br>	∧ -1.roles not in {EXPRESSION, STRING}<br>	∧ +1.reserved = ;<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 270.` |
| 26 | `  -1.reserved not in {;}<br>	∧ ^1.roles not in {QUALIFIED}<br>	∧ ^2.internal_type = VariableDeclaration<br>⇒ y = ␣<br>Confidence: 0.940. Support: 240.` |
| 27 | `  -1.internal_type = Identifier<br>	∧ -1.reserved not in {;}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.996. Support: 703.` |
| 28 | `  -1.internal_type not in {Identifier}<br>	∧ -1.reserved = (<br>	∧ -3.diff_line = 0<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 361.` |
| 29 | `  -1.internal_type not in {Identifier}<br>	∧ -1.reserved not in {(, ,, ;}<br>	∧ -3.diff_line = 0<br>	∧ +1.reserved = ;<br>	∧ +1.roles not in {LITERAL}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.966. Support: 309.` |
| 30 | `  -1.reserved not in {;}<br>	∧ +1.reserved = =<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.985. Support: 162.` |
| 31 | `  -1.reserved not in {;}<br>	∧ -1.roles in {EXPRESSION} and not in {STRING}<br>	∧ +1.reserved not in {=}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.964. Support: 1030.` |
| 32 | `  -1.reserved not in {;}<br>	∧ -1.roles not in {EXPRESSION, STRING}<br>	∧ +1.reserved = ;<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 283.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 5.59375, "max_conf": 0.9988937973976135, "max_support": 2217, "min_conf": 0.9260563254356384, "min_support": 142, "num_rules": 32}}
```
</details>
