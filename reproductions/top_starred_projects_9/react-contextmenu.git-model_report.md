# Model report for file:///tmp/top-repos-quality-repos-b1wc6w99/react-contextmenu.git HEAD d9018dbfbd6e21423cb2b753b3762adf5a6d77b0

### Dump

```json
{'created_at': '2021-08-29 11:48:41',
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
 'size': '17.0 kB',
 'tags': [],
 'uuid': 'fee39c01-0d25-422c-9354-7a9ecd731c37',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-b1wc6w99/react-contextmenu.git d9018dbfbd6e21423cb2b753b3762adf5a6d77b0

# javascript
74 rules, avg.len. 6.9
## train
PPCR: 0.934320
### report
macro
{'f1-score': 0.46672558997163877,
 'precision': 0.5165361256557045,
 'recall': 0.4412702351930335,
 'support': 10669}
micro
{'f1-score': 0.8667166557315588,
 'precision': 0.8667166557315588,
 'recall': 0.8667166557315588,
 'support': 10669}
weighted
{'f1-score': 0.845155265238669,
 'precision': 0.8345196287007698,
 'recall': 0.8667166557315588,
 'support': 10669}
### report_full
macro
{'f1-score': 0.45919667495614064,
 'precision': 0.5165361256557045,
 'recall': 0.4282673648137612,
 'support': 11419}
micro
{'f1-score': 0.8372872147772547,
 'precision': 0.8667166557315588,
 'recall': 0.8097906997110079,
 'support': 11419}
weighted
{'f1-score': 0.7941009754048655,
 'precision': 0.7899147629107997,
 'recall': 0.8097906997110079,
 'support': 11419}
## test
PPCR: 0.921518
### report
macro
{'f1-score': 0.4824865565273729,
 'precision': 0.515338535220945,
 'recall': 0.4593280610265961,
 'support': 2137}
micro
{'f1-score': 0.8427702386523164,
 'precision': 0.8427702386523164,
 'recall': 0.8427702386523164,
 'support': 2137}
weighted
{'f1-score': 0.8239094625839729,
 'precision': 0.8168545053085485,
 'recall': 0.8427702386523164,
 'support': 2137}
### report_full
macro
{'f1-score': 0.48015994727095357,
 'precision': 0.515338535220945,
 'recall': 0.4554839757341247,
 'support': 2319}
micro
{'f1-score': 0.8083482944344704,
 'precision': 0.8427702386523164,
 'recall': 0.7766278568348426,
 'support': 2319}
weighted
{'f1-score': 0.7615753352832498,
 'precision': 0.7584389446773407,
 'recall': 0.7766278568348426,
 'support': 2319}
```

## javascript
### Summary
33 rules, avg.len. 6.1

| | |
|-|-|
|Min support|145|
|Max support|1859|
|Min confidence|0.9206896424293518|
|Max confidence|0.9991735816001892|

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
| 1 | `  -1.internal_type = StringLiteral<br>⇒ y = '<br>Confidence: 0.998. Support: 289.` |
| 2 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label in {<space>}<br>	∧ +1.roles in {LITERAL}<br>⇒ y = '<br>Confidence: 0.997. Support: 187.` |
| 3 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = :<br>	∧ +1.roles in {LITERAL}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 179.` |
| 4 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;}<br>	∧ +1.roles not in {LITERAL}<br>	∧ +2.reserved = =<br>	∧ ^1.roles in {BLOCK} and not in {DECLARATION, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.960. Support: 186.` |
| 5 | `  ^1.roles in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 1364.` |
| 6 | `  +1.length ≥ 2<br>	∧ ^1.internal_type = JSXElement<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 433.` |
| 7 | `  -1.label in {<space>}<br>	∧ -1.reserved not in {(, ;}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = '<br>Confidence: 0.997. Support: 148.` |
| 8 | `  -1.internal_type = StringLiteral<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = '<br>Confidence: 0.998. Support: 236.` |
| 9 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = {<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles in {INCOMPLETE} and not in {QUALIFIED, VARIABLE}<br>⇒ y = ∅<br>Confidence: 0.949. Support: 205.` |
| 10 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {:}<br>	∧ -2.label in {<space>}<br>	∧ +1.roles in {LITERAL}<br>⇒ y = ␣<br>Confidence: 0.930. Support: 150.` |
| 11 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -2.internal_type = Identifier<br>	∧ -3.length ≤ 4<br>	∧ +1.roles not in {KEY}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.935. Support: 222.` |
| 12 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = {<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles in {INCOMPLETE} and not in {BINARY, QUALIFIED, VARIABLE}<br>⇒ y = ∅<br>Confidence: 0.925. Support: 194.` |
| 13 | `  ^1.internal_type = MemberExpression<br>⇒ y = ∅<br>Confidence: 0.997. Support: 1333.` |
| 14 | `  -1.label in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = '<br>Confidence: 0.997. Support: 156.` |
| 15 | `  -1.internal_type = StringLiteral<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = '<br>Confidence: 0.998. Support: 264.` |
| 16 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ;<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ⏎␣⁻␣⁻␣⁻␣⁻<br>Confidence: 0.923. Support: 162.` |
| 17 | `  •••start_line ≥ 12<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {INCOMPLETE} and not in {DECLARATION, OPERATOR}<br>	∧ ^2.roles not in {BODY}<br>⇒ y = ∅<br>Confidence: 0.985. Support: 1388.` |
| 18 | `  •••start_line ≥ 12<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {DECLARATION, INCOMPLETE, OPERATOR}<br>	∧ ^2.roles not in {BODY}<br>⇒ y = ∅<br>Confidence: 0.969. Support: 966.` |
| 19 | `  •••start_line ≥ 12<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = ;<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {DECLARATION, INCOMPLETE, OPERATOR}<br>	∧ ^2.roles not in {BODY}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 240.` |
| 20 | `  -1.label in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = '<br>Confidence: 0.997. Support: 157.` |
| 21 | `  -1.internal_type = StringLiteral<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = '<br>Confidence: 0.998. Support: 289.` |
| 22 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.internal_type = JSXElement<br>	∧ ^1.roles not in {QUALIFIED, VARIABLE}<br>⇒ y = ∅<br>Confidence: 0.965. Support: 1179.` |
| 23 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved = ;<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {QUALIFIED, VARIABLE}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 265.` |
| 24 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = =<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved not in {), }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {VARIABLE}<br>⇒ y = ∅<br>Confidence: 0.925. Support: 180.` |
| 25 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;, =}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ -3.label in {<space>}<br>	∧ +1.reserved not in {), ;, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {LITERAL, QUALIFIED, VARIABLE}<br>⇒ y = ␣<br>Confidence: 0.954. Support: 164.` |
| 26 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.roles in {OPERATOR} and not in {QUALIFIED, VARIABLE}<br>⇒ y = ␣<br>Confidence: 0.921. Support: 145.` |
| 27 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 605.` |
| 28 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type = JSXElement<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.993. Support: 477.` |
| 29 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label in {<space>}<br>	∧ -1.reserved not in {(, ;}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = '<br>Confidence: 0.997. Support: 152.` |
| 30 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -3.length ≥ 3<br>	∧ +1.roles not in {KEY}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.929. Support: 642.` |
| 31 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {VARIABLE}<br>⇒ y = ∅<br>Confidence: 0.980. Support: 1859.` |
| 32 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles in {INCOMPLETE} and not in {VARIABLE}<br>⇒ y = ∅<br>Confidence: 0.991. Support: 616.` |
| 33 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = ;<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {INCOMPLETE, VARIABLE}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 272.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 6.090909090909091, "max_conf": 0.9991735816001892, "max_support": 1859, "min_conf": 0.9206896424293518, "min_support": 145, "num_rules": 33}}
```
</details>
