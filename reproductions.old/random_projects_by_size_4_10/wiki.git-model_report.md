# Model report for file:///tmp/top-repos-quality-repos-zeo4ippp/wiki.git HEAD ebe4f45a9c9c77835430d03a59b5b2ec43a37071

### Dump

```json
{'created_at': '2021-08-21 20:35:06',
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
 'size': '23.3 kB',
 'tags': [],
 'uuid': '8b177171-c03f-4001-a7c6-6d406270397d',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-zeo4ippp/wiki.git ebe4f45a9c9c77835430d03a59b5b2ec43a37071

# javascript
193 rules, avg.len. 8.8
## train
PPCR: 0.981494
### report
macro
{'f1-score': 0.7350407964630363,
 'precision': 0.7747056986542007,
 'recall': 0.715017658727409,
 'support': 37763}
micro
{'f1-score': 0.9386966077906946,
 'precision': 0.9386966077906946,
 'recall': 0.9386966077906946,
 'support': 37763}
weighted
{'f1-score': 0.935107162208348,
 'precision': 0.9343244059949469,
 'recall': 0.9386966077906946,
 'support': 37763}
### report_full
macro
{'f1-score': 0.7161210509514339,
 'precision': 0.7747056986542007,
 'recall': 0.6883893610918368,
 'support': 38475}
micro
{'f1-score': 0.9299299561898265,
 'precision': 0.9386966077906946,
 'recall': 0.9213255360623782,
 'support': 38475}
weighted
{'f1-score': 0.9248984903903026,
 'precision': 0.934313789898043,
 'recall': 0.9213255360623782,
 'support': 38475}
## test
PPCR: 0.962200
### report
macro
{'f1-score': 0.5835503487758167,
 'precision': 0.6325995240069059,
 'recall': 0.587104782426434,
 'support': 5371}
micro
{'f1-score': 0.8588717184881772,
 'precision': 0.8588717184881772,
 'recall': 0.8588717184881772,
 'support': 5371}
weighted
{'f1-score': 0.85143933761354,
 'precision': 0.8559310116114508,
 'recall': 0.8588717184881772,
 'support': 5371}
### report_full
macro
{'f1-score': 0.5653578682129544,
 'precision': 0.6325995240069059,
 'recall': 0.5566320427935931,
 'support': 5582}
micro
{'f1-score': 0.8423263032959007,
 'precision': 0.8588717184881772,
 'recall': 0.8264063059835185,
 'support': 5582}
weighted
{'f1-score': 0.8299475977544793,
 'precision': 0.8550881714054179,
 'recall': 0.8264063059835185,
 'support': 5582}
```

## javascript
### Summary
136 rules, avg.len. 8.4

| | |
|-|-|
|Min support|146|
|Max support|5899|
|Min confidence|0.9214689135551453|
|Max confidence|0.9997124671936035|

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
| 1 | `  ^1.roles in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.990. Support: 3407.` |
| 2 | `  -1.internal_type = StringLiteral<br>	∧ -4.roles not in {STRING}<br>	∧ ^1.roles not in {IDENTIFIER}<br>	∧ ^2.roles in {MAP}<br>⇒ y = '<br>Confidence: 0.959. Support: 1534.` |
| 3 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.973. Support: 1482.` |
| 4 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {{}<br>	∧ +1.reserved = }<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.958. Support: 1495.` |
| 5 | `  -1.diff_offset ≥ 6<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {{}<br>	∧ -3.diff_line ≥ 1<br>	∧ +1.internal_type = StringLiteral<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles in {KEY}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = '<br>Confidence: 0.997. Support: 818.` |
| 6 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {{}<br>	∧ -3.diff_line = 0<br>	∧ +1.internal_type = StringLiteral<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles in {KEY}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ⏎<br>Confidence: 0.999. Support: 647.` |
| 7 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label in {<space>}<br>	∧ -1.reserved not in {{}<br>	∧ -4.length ≥ 6<br>	∧ +1.internal_type = StringLiteral<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = '<br>Confidence: 0.936. Support: 646.` |
| 8 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {{}<br>	∧ -5.length ≤ 10<br>	∧ +1.internal_type = StringLiteral<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.948. Support: 1608.` |
| 9 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {{}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = =<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 254.` |
| 10 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {{}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, }}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.988. Support: 3781.` |
| 11 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {{}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ ^1.internal_type = ObjectProperty<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.982. Support: 1500.` |
| 12 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ ^1.internal_type not in {ObjectProperty}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 1220.` |
| 13 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles in {EXPRESSION, KEY}<br>	∧ ^1.internal_type not in {ObjectProperty}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ⏎<br>Confidence: 0.989. Support: 423.` |
| 14 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = {<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.internal_type not in {ObjectProperty}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.942. Support: 819.` |
| 15 | `  -1.internal_type = CommentLine<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.internal_type not in {ObjectProperty}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ⏎<br>Confidence: 0.930. Support: 336.` |
| 16 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {(, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≥ 2<br>	∧ +2.roles not in {COMMENT}<br>	∧ ^1.internal_type not in {IfStatement, ObjectProperty}<br>	∧ ^1.roles not in {OPERATOR, SWITCH}<br>⇒ y = ∅<br>Confidence: 0.935. Support: 864.` |
| 17 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {(, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ +2.roles not in {COMMENT}<br>	∧ ^1.internal_type not in {IfStatement, ObjectProperty}<br>	∧ ^1.roles not in {OPERATOR, SWITCH}<br>⇒ y = ∅<br>Confidence: 0.985. Support: 1561.` |
| 18 | `  ^1.roles in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.985. Support: 3453.` |
| 19 | `  -1.internal_type = StringLiteral<br>	∧ -4.internal_type not in {StringLiteral}<br>	∧ ^1.roles not in {QUALIFIED}<br>	∧ ^2.roles in {MAP}<br>⇒ y = '<br>Confidence: 0.953. Support: 1504.` |
| 20 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.966. Support: 1502.` |
| 21 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label in {<space>}<br>	∧ -1.reserved not in {{}<br>	∧ -4.length ≥ 6<br>	∧ +1.internal_type = StringLiteral<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = '<br>Confidence: 0.930. Support: 697.` |
| 22 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {{}<br>	∧ -2.diff_line ≥ 1<br>	∧ -2.diff_offset ≥ 7<br>	∧ +1.internal_type = StringLiteral<br>	∧ +1.roles in {KEY}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = '<br>Confidence: 0.996. Support: 826.` |
| 23 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {{}<br>	∧ -2.diff_line = 0<br>	∧ +1.internal_type = StringLiteral<br>	∧ +1.roles in {KEY}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ⏎<br>Confidence: 0.999. Support: 661.` |
| 24 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {{}<br>	∧ -4.internal_type not in {Identifier}<br>	∧ +1.internal_type = StringLiteral<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.971. Support: 1622.` |
| 25 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {{}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = }<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.953. Support: 1507.` |
| 26 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {{}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = {<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.927. Support: 1547.` |
| 27 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 895.` |
| 28 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.roles in {KEY} and not in {COMMENT}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ⏎<br>Confidence: 0.980. Support: 415.` |
| 29 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles in {BLOCK}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.roles not in {EXPRESSION, KEY}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 275.` |
| 30 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {BLOCK}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.roles not in {COMMENT, EXPRESSION, KEY}<br>	∧ +1.length ≥ 2<br>	∧ +2.roles in {CASE}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ⏎⏎␣⁻␣⁻<br>Confidence: 0.952. Support: 200.` |
| 31 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {{}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>	∧ ^2.internal_type = VariableDeclaration<br>⇒ y = ␣<br>Confidence: 0.967. Support: 347.` |
| 32 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {{}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.length ≤ 1<br>	∧ +2.internal_type = CommentBlock<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ⏎<br>Confidence: 0.955. Support: 213.` |
| 33 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ +2.internal_type not in {CommentBlock}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.990. Support: 156.` |
| 34 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {if, {}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.954. Support: 5899.` |
| 35 | `  ^1.internal_type = MemberExpression<br>⇒ y = ∅<br>Confidence: 0.985. Support: 3520.` |
| 36 | `  -1.internal_type = StringLiteral<br>	∧ -4.roles not in {STRING}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^2.roles in {MAP}<br>⇒ y = '<br>Confidence: 0.957. Support: 1531.` |
| 37 | `  -1.internal_type = StringLiteral<br>	∧ -5.length ≤ 4<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^2.roles not in {MAP}<br>⇒ y = "<br>Confidence: 0.921. Support: 885.` |
| 38 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.963. Support: 1486.` |
| 39 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {{}<br>	∧ +1.reserved = }<br>	∧ +1.roles not in {STRING}<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.964. Support: 1518.` |
| 40 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label in {<space>}<br>	∧ -1.reserved not in {{}<br>	∧ -4.length ≥ 6<br>	∧ +1.internal_type = StringLiteral<br>	∧ +1.reserved not in {}}<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = '<br>Confidence: 0.955. Support: 684.` |
| 41 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {{}<br>	∧ -2.diff_line ≥ 1<br>	∧ -2.diff_offset ≥ 7<br>	∧ +1.internal_type = StringLiteral<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles in {KEY}<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = '<br>Confidence: 0.994. Support: 756.` |
| 42 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {{}<br>	∧ -2.diff_line = 0<br>	∧ +1.internal_type = StringLiteral<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles in {KEY}<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ⏎<br>Confidence: 0.999. Support: 649.` |
| 43 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {{}<br>	∧ -4.internal_type not in {Identifier}<br>	∧ +1.internal_type = StringLiteral<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ␣<br>Confidence: 0.976. Support: 1494.` |
| 44 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {{}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = =<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 286.` |
| 45 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {{}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, }}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.989. Support: 3733.` |
| 46 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {{}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ ^1.internal_type = ObjectProperty<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.983. Support: 1444.` |
| 47 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ ^1.internal_type not in {MemberExpression, ObjectProperty}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 1161.` |
| 48 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles in {EXPRESSION, KEY}<br>	∧ ^1.internal_type not in {MemberExpression, ObjectProperty}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ⏎<br>Confidence: 0.990. Support: 455.` |
| 49 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles in {EXPRESSION} and not in {KEY}<br>	∧ ^1.internal_type not in {MemberExpression, ObjectProperty}<br>	∧ ^1.roles in {EXPRESSION} and not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.966. Support: 544.` |
| 50 | `  -1.diff_col ≤ 1<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = =<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles in {EXPRESSION} and not in {KEY}<br>	∧ ^1.internal_type not in {MemberExpression, ObjectProperty}<br>	∧ ^1.roles not in {EXPRESSION, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 152.` |
| 51 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = {<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.internal_type not in {MemberExpression, ObjectProperty}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.955. Support: 817.` |
| 52 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ;<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +2.reserved = (<br>	∧ ^1.internal_type not in {MemberExpression, ObjectProperty}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ⏎⏎<br>Confidence: 0.963. Support: 150.` |
| 53 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = ;<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +2.roles not in {COMMENT}<br>	∧ ^1.internal_type not in {IfStatement, ObjectProperty}<br>	∧ ^1.roles not in {OPERATOR, SWITCH}<br>	∧ ^2.roles in {DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 146.` |
| 54 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {(, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +2.roles not in {COMMENT}<br>	∧ ^1.internal_type not in {IfStatement, ObjectProperty}<br>	∧ ^1.roles not in {OPERATOR, SWITCH}<br>	∧ ^2.roles not in {DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.946. Support: 2339.` |
| 55 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {{}<br>	∧ +1.reserved = }<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.967. Support: 1483.` |
| 56 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ +1.internal_type = StringLiteral<br>	∧ +1.reserved not in {}}<br>	∧ ^1.internal_type = ObjectExpression<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ⏎<br>Confidence: 0.999. Support: 612.` |
| 57 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ +1.internal_type = StringLiteral<br>	∧ +1.reserved not in {}}<br>	∧ ^1.internal_type not in {ObjectExpression}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 375.` |
| 58 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = :<br>	∧ +1.internal_type = StringLiteral<br>	∧ +1.reserved not in {}}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 614.` |
| 59 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, {}<br>	∧ -3.internal_type not in {StringLiteral}<br>	∧ +1.internal_type = StringLiteral<br>	∧ +1.reserved not in {}}<br>	∧ ^1.roles not in {QUALIFIED}<br>	∧ ^2.roles in {MAP}<br>⇒ y = '<br>Confidence: 0.967. Support: 1497.` |
| 60 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label in {<space>}<br>	∧ -1.reserved not in {,, :, {}<br>	∧ -4.length ≤ 3<br>	∧ +1.internal_type = StringLiteral<br>	∧ +1.reserved not in {}}<br>	∧ ^1.roles not in {QUALIFIED}<br>	∧ ^2.roles not in {MAP}<br>⇒ y = "<br>Confidence: 0.923. Support: 652.` |
| 61 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {,, :, {}<br>	∧ -2.diff_col ≥ 5<br>	∧ -2.diff_offset ≤ 6<br>	∧ +1.internal_type = StringLiteral<br>	∧ +1.reserved not in {}}<br>	∧ ^1.roles not in {QUALIFIED}<br>	∧ ^2.roles not in {MAP}<br>⇒ y = "<br>Confidence: 0.924. Support: 165.` |
| 62 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {{}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = =<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 250.` |
| 63 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {{}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, }}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.991. Support: 3777.` |
| 64 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {{}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ ^1.internal_type = ObjectProperty<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.981. Support: 1469.` |
| 65 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ ^1.internal_type not in {ObjectProperty}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 1173.` |
| 66 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles in {EXPRESSION, KEY}<br>	∧ ^1.internal_type not in {ObjectProperty}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ⏎<br>Confidence: 0.986. Support: 452.` |
| 67 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = {<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.internal_type not in {ObjectProperty}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.939. Support: 848.` |
| 68 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {{}<br>	∧ -4.length ≥ 5<br>	∧ +1.internal_type = StringLiteral<br>	∧ +1.reserved not in {}}<br>	∧ ^1.roles in {MAP} and not in {QUALIFIED}<br>⇒ y = '<br>Confidence: 0.964. Support: 1196.` |
| 69 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {{}<br>	∧ -2.diff_line = 0<br>	∧ -4.length ≤ 4<br>	∧ +1.internal_type = StringLiteral<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles in {KEY}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ⏎<br>Confidence: 0.999. Support: 642.` |
| 70 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {{}<br>	∧ -4.length ≤ 4<br>	∧ +1.internal_type = StringLiteral<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.975. Support: 1282.` |
| 71 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {{}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>	∧ ^2.internal_type = ObjectExpression<br>⇒ y = ␣<br>Confidence: 0.980. Support: 1456.` |
| 72 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>	∧ ^2.internal_type not in {ObjectExpression}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 1200.` |
| 73 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles in {EXPRESSION, KEY}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>	∧ ^2.internal_type not in {ObjectExpression}<br>⇒ y = ⏎<br>Confidence: 0.970. Support: 447.` |
| 74 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles in {EXPRESSION} and not in {KEY}<br>	∧ ^1.roles in {EXPRESSION} and not in {OPERATOR, QUALIFIED}<br>	∧ ^2.internal_type not in {ObjectExpression}<br>⇒ y = ␣<br>Confidence: 0.953. Support: 582.` |
| 75 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = =<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≤ 1<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles in {EXPRESSION} and not in {KEY}<br>	∧ ^1.roles not in {EXPRESSION, OPERATOR, QUALIFIED}<br>	∧ ^2.internal_type not in {ObjectExpression}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 171.` |
| 76 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = {<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>	∧ ^2.internal_type not in {ObjectExpression}<br>⇒ y = ␣<br>Confidence: 0.943. Support: 781.` |
| 77 | `  -1.internal_type = CommentLine<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +2.roles in {COMMENT}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>	∧ ^2.internal_type not in {ObjectExpression}<br>⇒ y = ⏎<br>Confidence: 0.943. Support: 220.` |
| 78 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.diff_offset ≤ 5<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {(, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≥ 2<br>	∧ +2.roles not in {COMMENT}<br>	∧ ^1.internal_type = File<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {ObjectExpression}<br>⇒ y = ∅<br>Confidence: 0.970. Support: 658.` |
| 79 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {(, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ +2.roles not in {COMMENT}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {ObjectExpression}<br>⇒ y = ∅<br>Confidence: 0.972. Support: 1789.` |
| 80 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {{}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = }<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.959. Support: 1529.` |
| 81 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label in {<newline>} and not in {<space>}<br>	∧ -1.reserved not in {{}<br>	∧ -2.diff_offset ≥ 7<br>	∧ +1.internal_type = StringLiteral<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles in {KEY}<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = '<br>Confidence: 0.998. Support: 775.` |
| 82 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<newline>, <space>}<br>	∧ -1.reserved not in {{}<br>	∧ +1.internal_type = StringLiteral<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles in {KEY}<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ⏎<br>Confidence: 0.999. Support: 641.` |
| 83 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {{}<br>	∧ -4.roles not in {IDENTIFIER}<br>	∧ +1.internal_type = StringLiteral<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ␣<br>Confidence: 0.976. Support: 1498.` |
| 84 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = {<br>	∧ ^1.internal_type not in {MemberExpression, ObjectProperty}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.954. Support: 880.` |
| 85 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.roles in {EXPRESSION, KEY}<br>	∧ ^1.internal_type not in {MemberExpression, ObjectProperty}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ⏎<br>Confidence: 0.986. Support: 466.` |
| 86 | `  -1.internal_type = CommentLine<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.internal_type not in {MemberExpression, ObjectProperty}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ⏎<br>Confidence: 0.944. Support: 310.` |
| 87 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {(, ;}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -3.diff_line ≥ 1<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = (<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.internal_type not in {MemberExpression, ObjectProperty}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.977. Support: 281.` |
| 88 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {(, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {IfStatement, ObjectProperty}<br>	∧ ^1.roles not in {OPERATOR, SWITCH}<br>⇒ y = ∅<br>Confidence: 0.963. Support: 1602.` |
| 89 | `  -1.reserved = {<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.942. Support: 1627.` |
| 90 | `  -1.internal_type = StringLiteral<br>	∧ -1.reserved not in {{}<br>	∧ -4.roles not in {STRING}<br>	∧ ^1.roles not in {IDENTIFIER}<br>	∧ ^2.roles in {MAP}<br>⇒ y = '<br>Confidence: 0.963. Support: 1501.` |
| 91 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {{}<br>	∧ +1.reserved = }<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.973. Support: 1521.` |
| 92 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {{}<br>	∧ -3.diff_line = 0<br>	∧ +1.internal_type = StringLiteral<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles in {KEY}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ⏎<br>Confidence: 0.999. Support: 632.` |
| 93 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {{}<br>	∧ -4.roles not in {IDENTIFIER}<br>	∧ +1.internal_type = StringLiteral<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.973. Support: 1478.` |
| 94 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {{}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>	∧ ^2.internal_type = ObjectExpression<br>⇒ y = ␣<br>Confidence: 0.986. Support: 1438.` |
| 95 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>	∧ ^2.internal_type not in {ObjectExpression}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 1214.` |
| 96 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles in {EXPRESSION, KEY}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>	∧ ^2.internal_type not in {ObjectExpression}<br>⇒ y = ⏎<br>Confidence: 0.972. Support: 452.` |
| 97 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = {<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>	∧ ^2.internal_type not in {ObjectExpression}<br>⇒ y = ␣<br>Confidence: 0.925. Support: 843.` |
| 98 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {(, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {IfStatement}<br>	∧ ^1.roles not in {OPERATOR, SWITCH}<br>	∧ ^2.internal_type not in {ObjectExpression}<br>⇒ y = ∅<br>Confidence: 0.962. Support: 1656.` |
| 99 | `  -1.internal_type = StringLiteral<br>	∧ -4.internal_type not in {StringLiteral}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^2.roles in {MAP}<br>⇒ y = '<br>Confidence: 0.948. Support: 1521.` |
| 100 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ +1.internal_type = StringLiteral<br>	∧ +1.reserved not in {}}<br>	∧ +2.reserved = :<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ⏎<br>Confidence: 0.999. Support: 661.` |
| 101 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ +1.internal_type = StringLiteral<br>	∧ +1.reserved not in {}}<br>	∧ +2.reserved not in {:}<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 345.` |
| 102 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = :<br>	∧ +1.internal_type = StringLiteral<br>	∧ +1.reserved not in {}}<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 664.` |
| 103 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {,, :, {}<br>	∧ -2.diff_col ≥ 5<br>	∧ -2.diff_offset ≤ 6<br>	∧ +1.internal_type = StringLiteral<br>	∧ +1.reserved not in {}}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^2.roles not in {MAP}<br>⇒ y = "<br>Confidence: 0.943. Support: 149.` |
| 104 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {{}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type = ObjectExpression<br>⇒ y = ␣<br>Confidence: 0.980. Support: 1509.` |
| 105 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {ObjectExpression}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 1196.` |
| 106 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles in {EXPRESSION, KEY}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {ObjectExpression}<br>⇒ y = ⏎<br>Confidence: 0.986. Support: 466.` |
| 107 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = {<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {ObjectExpression}<br>⇒ y = ␣<br>Confidence: 0.936. Support: 825.` |
| 108 | `  -1.internal_type = CommentLine<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {ObjectExpression}<br>⇒ y = ⏎<br>Confidence: 0.947. Support: 309.` |
| 109 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {(, ;}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -3.diff_line ≥ 1<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = (<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {ObjectExpression}<br>⇒ y = ␣<br>Confidence: 0.984. Support: 283.` |
| 110 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = ;<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +2.roles not in {COMMENT}<br>	∧ ^1.internal_type not in {IfStatement}<br>	∧ ^1.roles not in {OPERATOR, SWITCH}<br>	∧ ^2.internal_type not in {ObjectExpression}<br>	∧ ^2.roles in {DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 183.` |
| 111 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {(, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +2.roles not in {COMMENT}<br>	∧ ^1.internal_type not in {IfStatement}<br>	∧ ^1.roles not in {OPERATOR, SWITCH}<br>	∧ ^2.internal_type not in {ObjectExpression}<br>	∧ ^2.roles not in {DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.944. Support: 2266.` |
| 112 | `  -1.roles in {IDENTIFIER}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.950. Support: 793.` |
| 113 | `  -1.roles in {IDENTIFIER}<br>	∧ +1.reserved = =<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 277.` |
| 114 | `  -1.roles in {IDENTIFIER}<br>	∧ +1.reserved not in {=}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.986. Support: 5065.` |
| 115 | `  -1.internal_type = StringLiteral<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ -4.roles not in {STRING}<br>	∧ ^2.roles in {MAP}<br>⇒ y = '<br>Confidence: 0.949. Support: 1565.` |
| 116 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ -1.roles not in {IDENTIFIER}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.972. Support: 1472.` |
| 117 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {{}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved = }<br>	∧ +1.roles not in {STRING}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.969. Support: 1446.` |
| 118 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label in {<space>}<br>	∧ -1.reserved not in {{}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ -4.length ≥ 6<br>	∧ +1.internal_type = StringLiteral<br>	∧ +1.reserved not in {}}<br>⇒ y = '<br>Confidence: 0.951. Support: 709.` |
| 119 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {{}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ -2.diff_offset ≥ 7<br>	∧ -3.diff_line ≥ 1<br>	∧ +1.internal_type = StringLiteral<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles in {KEY}<br>⇒ y = '<br>Confidence: 0.992. Support: 785.` |
| 120 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {{}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ -3.diff_line = 0<br>	∧ +1.internal_type = StringLiteral<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles in {KEY}<br>⇒ y = ⏎<br>Confidence: 0.999. Support: 624.` |
| 121 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {{}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ -4.internal_type not in {Identifier}<br>	∧ +1.internal_type = StringLiteral<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {KEY}<br>⇒ y = ␣<br>Confidence: 0.957. Support: 1485.` |
| 122 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {{}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ ^2.internal_type = ObjectExpression<br>⇒ y = ␣<br>Confidence: 0.986. Support: 1448.` |
| 123 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {{}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ ^1.roles in {IDENTIFIER}<br>	∧ ^2.internal_type not in {ObjectExpression}<br>⇒ y = ∅<br>Confidence: 1.000. Support: 1739.` |
| 124 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ ^1.roles not in {IDENTIFIER}<br>	∧ ^2.internal_type not in {ObjectExpression}<br>⇒ y = ∅<br>Confidence: 0.995. Support: 1267.` |
| 125 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles in {EXPRESSION, KEY}<br>	∧ ^1.roles not in {IDENTIFIER}<br>	∧ ^2.internal_type not in {ObjectExpression}<br>⇒ y = ⏎<br>Confidence: 0.981. Support: 437.` |
| 126 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = {<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.roles not in {IDENTIFIER}<br>	∧ ^2.internal_type not in {ObjectExpression}<br>⇒ y = ␣<br>Confidence: 0.938. Support: 874.` |
| 127 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ;<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +2.reserved = (<br>	∧ ^1.roles not in {IDENTIFIER}<br>	∧ ^2.internal_type not in {ObjectExpression}<br>⇒ y = ⏎⏎<br>Confidence: 0.967. Support: 165.` |
| 128 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {(, ;}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ -2.label in {<newline>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = (<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>	∧ ^2.internal_type not in {ObjectExpression}<br>⇒ y = ␣<br>Confidence: 0.984. Support: 280.` |
| 129 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {(, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type = IfStatement<br>	∧ ^1.roles not in {FILE, IDENTIFIER, OPERATOR}<br>	∧ ^2.internal_type not in {ObjectExpression}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 166.` |
| 130 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {(, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {ObjectExpression}<br>⇒ y = ∅<br>Confidence: 0.952. Support: 2325.` |
| 131 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {{}<br>	∧ +1.reserved = }<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.961. Support: 1487.` |
| 132 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ +1.internal_type = StringLiteral<br>	∧ +1.reserved not in {}}<br>	∧ +2.reserved = :<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ⏎<br>Confidence: 0.999. Support: 640.` |
| 133 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ +1.internal_type = StringLiteral<br>	∧ +1.reserved not in {}}<br>	∧ +2.reserved not in {:}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 376.` |
| 134 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, {}<br>	∧ -3.roles not in {STRING}<br>	∧ +1.internal_type = StringLiteral<br>	∧ +1.reserved not in {}}<br>	∧ ^1.roles not in {QUALIFIED}<br>	∧ ^2.roles in {MAP}<br>⇒ y = '<br>Confidence: 0.961. Support: 1547.` |
| 135 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {{}<br>	∧ -5.internal_type not in {Identifier}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ ^1.roles in {OPERATOR} and not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.962. Support: 1468.` |
| 136 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles in {COMMENT} and not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +2.roles in {COMMENT}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>	∧ ^2.internal_type not in {ObjectExpression}<br>⇒ y = ⏎<br>Confidence: 0.958. Support: 226.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 8.411764705882353, "max_conf": 0.9997124671936035, "max_support": 5899, "min_conf": 0.9214689135551453, "min_support": 146, "num_rules": 136}}
```
</details>
