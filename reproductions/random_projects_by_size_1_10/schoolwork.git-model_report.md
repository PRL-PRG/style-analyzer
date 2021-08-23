# Model report for file:///tmp/top-repos-quality-repos-u09lph72/schoolwork.git HEAD 1afb6b507d106a90c2493e4b4bb632a160dfe3e5

### Dump

```json
{'created_at': '2021-08-22 10:32:12',
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
 'size': '24.8 kB',
 'tags': [],
 'uuid': 'a0d19086-8937-4846-9841-bca6894f79b5',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-u09lph72/schoolwork.git 1afb6b507d106a90c2493e4b4bb632a160dfe3e5

# javascript
104 rules, avg.len. 11.0
## train
PPCR: 0.849800
### report
macro
{'f1-score': 0.6477886516127055,
 'precision': 0.6661991256316836,
 'recall': 0.6337343954736175,
 'support': 163617}
micro
{'f1-score': 0.9617093578295653,
 'precision': 0.9617093578295653,
 'recall': 0.9617093578295653,
 'support': 163617}
weighted
{'f1-score': 0.9604350363115444,
 'precision': 0.9598444396327088,
 'recall': 0.9617093578295653,
 'support': 163617}
### report_full
macro
{'f1-score': 0.5002111026953653,
 'precision': 0.6661991256316836,
 'recall': 0.42168709904402607,
 'support': 192536}
micro
{'f1-score': 0.8836202418623457,
 'precision': 0.9617093578295653,
 'recall': 0.8172601487514023,
 'support': 192536}
weighted
{'f1-score': 0.8686480229417273,
 'precision': 0.9528860851119486,
 'recall': 0.8172601487514023,
 'support': 192536}
## test
PPCR: 0.875359
### report
macro
{'f1-score': 0.5234437563620741,
 'precision': 0.5333186033086162,
 'recall': 0.5153031996759936,
 'support': 29244}
micro
{'f1-score': 0.9728149363972097,
 'precision': 0.9728149363972097,
 'recall': 0.9728149363972097,
 'support': 29244}
weighted
{'f1-score': 0.9722369503126712,
 'precision': 0.9722080738217665,
 'recall': 0.9728149363972097,
 'support': 29244}
### report_full
macro
{'f1-score': 0.40136338100314545,
 'precision': 0.5333186033086162,
 'recall': 0.3390806996891609,
 'support': 33408}
micro
{'f1-score': 0.9081593564451255,
 'precision': 0.9728149363972097,
 'recall': 0.8515625,
 'support': 33408}
weighted
{'f1-score': 0.8957040710139499,
 'precision': 0.9681059734042824,
 'recall': 0.8515625,
 'support': 33408}
```

## javascript
### Summary
45 rules, avg.len. 10.0

| | |
|-|-|
|Min support|90|
|Max support|19243|
|Min confidence|0.9202127456665039|
|Max confidence|0.9999529719352722|

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
| 1 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {)}<br>	∧ +1.roles not in {LITERAL}<br>	∧ +2.roles in {EXPRESSION}<br>	∧ ^1.roles in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 12705.` |
| 2 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = .<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {)}<br>	∧ +1.roles not in {LITERAL}<br>	∧ +2.roles not in {EXPRESSION}<br>	∧ ^1.roles in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 1.000. Support: 10637.` |
| 3 | `  •••start_col ≥ 86<br>	∧ -1.label not in {<space>}<br>	∧ -1.roles in {STRING}<br>	∧ -3.length ≥ 3<br>	∧ ^1.roles in {OPERATOR} and not in {QUALIFIED}<br>⇒ y = '<br>Confidence: 0.941. Support: 110.` |
| 4 | `  -1.label not in {<space>}<br>	∧ -1.roles not in {STRING}<br>	∧ -2.reserved = [<br>	∧ ^1.roles in {OPERATOR} and not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.973. Support: 530.` |
| 5 | `  -1.label not in {<space>}<br>	∧ -1.roles not in {STRING}<br>	∧ -2.label not in {<space>}<br>	∧ -2.reserved not in {[}<br>	∧ -4.label not in {<space>}<br>	∧ +1.reserved = )<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.944. Support: 314.` |
| 6 | `  -1.label not in {<space>}<br>	∧ -1.roles not in {STRING}<br>	∧ -2.reserved not in {[}<br>	∧ -4.diff_offset ≥ 7<br>	∧ +1.reserved not in {(, )}<br>	∧ ^1.roles in {OPERATOR} and not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.985. Support: 14561.` |
| 7 | `  -1.label not in {<space>}<br>	∧ -1.roles not in {STRING}<br>	∧ -2.reserved not in {[}<br>	∧ -3.roles in {EXPRESSION}<br>	∧ -3.length ≥ 2<br>	∧ -4.diff_offset ≤ 6<br>	∧ +1.reserved not in {)}<br>	∧ ^1.roles in {OPERATOR} and not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.990. Support: 261.` |
| 8 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles not in {STRING}<br>	∧ -2.reserved not in {[}<br>	∧ -3.reserved not in {=}<br>	∧ -3.length ≤ 1<br>	∧ -4.diff_offset ≤ 6<br>	∧ +1.reserved not in {)}<br>	∧ ^1.roles in {OPERATOR} and not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.922. Support: 2594.` |
| 9 | `  -1.roles in {EXPRESSION}<br>	∧ +1.reserved = =<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 1.000. Support: 2938.` |
| 10 | `  -1.internal_type = StringLiteral<br>	∧ -1.roles in {EXPRESSION}<br>	∧ -2.label in {<newline>}<br>	∧ +1.reserved not in {=}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = "<br>Confidence: 0.961. Support: 141.` |
| 11 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ -2.label in {<space>}<br>	∧ -3.reserved = (<br>	∧ +1.reserved = )<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 983.` |
| 12 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ -2.label in {<space>}<br>	∧ -3.reserved not in {(}<br>	∧ -4.roles in {EXPRESSION}<br>	∧ -5.label not in {<space>}<br>	∧ +1.reserved = )<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.973. Support: 425.` |
| 13 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ -2.label not in {<space>}<br>	∧ -2.reserved = (<br>	∧ +1.reserved = )<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 1.000. Support: 1150.` |
| 14 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.reserved not in {), =}<br>	∧ +2.roles in {EXPRESSION}<br>	∧ ^1.internal_type = ConditionalExpression<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 771.` |
| 15 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ -2.label not in {<space>}<br>	∧ +1.reserved not in {), =, }}<br>	∧ +1.length ≤ 2<br>	∧ +2.reserved = ;<br>	∧ ^1.internal_type not in {ConditionalExpression}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.961. Support: 298.` |
| 16 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.reserved not in {), =, }}<br>	∧ +1.length ≤ 2<br>	∧ +2.reserved not in {;}<br>	∧ ^1.internal_type not in {ConditionalExpression}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.983. Support: 17379.` |
| 17 | `  -1.internal_type = CommentLine<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ⏎<br>Confidence: 0.968. Support: 3231.` |
| 18 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.reserved = ;<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -3.diff_offset ≥ 6<br>	∧ -3.label in {<space>}<br>	∧ +1.reserved = }<br>	∧ +3.length ≥ 97<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.983. Support: 147.` |
| 19 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.reserved = ;<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -3.diff_offset ≥ 6<br>	∧ -3.label in {<space>}<br>	∧ -5.label in {<+space>}<br>	∧ +1.reserved = }<br>	∧ +3.length ≤ 97<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.996. Support: 121.` |
| 20 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.reserved = ;<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -3.label not in {<-tab>, <space>}<br>	∧ -3.roles in {CALL}<br>	∧ +1.reserved = }<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.998. Support: 277.` |
| 21 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.reserved = ;<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {}}<br>	∧ +1.length ≤ 2<br>	∧ ^1.roles in {SCOPE} and not in {OPERATOR, QUALIFIED}<br>	∧ ^2.roles in {ASSIGNMENT}<br>⇒ y = ⏎⏎<br>Confidence: 0.928. Support: 90.` |
| 22 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.reserved = {<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -3.reserved = )<br>	∧ -4.label in {<space>}<br>	∧ +2.roles not in {COMMENT}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ⏎⇥⁺<br>Confidence: 0.946. Support: 1510.` |
| 23 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.reserved = {<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -4.label not in {<space>}<br>	∧ -5.reserved not in {function}<br>	∧ +1.roles not in {MAP}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.955. Support: 2216.` |
| 24 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.reserved not in {;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = ;<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.986. Support: 4262.` |
| 25 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.reserved not in {;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {;}<br>	∧ +1.roles in {COMMENT}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 1.000. Support: 4073.` |
| 26 | `  •••start_col ≥ 26<br>	∧ -1.internal_type not in {CommentLine}<br>	∧ -1.reserved = (<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.label in {<space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles in {ARGUMENT}<br>	∧ +1.length ≤ 5<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.996. Support: 139.` |
| 27 | `  •••start_col ≥ 26<br>	∧ -1.internal_type not in {CommentLine}<br>	∧ -1.reserved = (<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.diff_offset ≤ 9<br>	∧ -2.label not in {<space>}<br>	∧ -2.roles in {IDENTIFIER}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles in {ARGUMENT} and not in {COMMENT}<br>	∧ +1.length ≤ 5<br>	∧ +3.roles in {EXPRESSION}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.958. Support: 155.` |
| 28 | `  •••start_col ≥ 26<br>	∧ -1.internal_type not in {CommentLine}<br>	∧ -1.reserved = (<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.label not in {<space>}<br>	∧ -2.roles not in {IDENTIFIER}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles in {ARGUMENT} and not in {COMMENT}<br>	∧ +1.length ≤ 5<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.936. Support: 118.` |
| 29 | `  •••start_col ≤ 25<br>	∧ •••start_line ≥ 252<br>	∧ -1.internal_type not in {CommentLine}<br>	∧ -1.reserved = (<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = )<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 249.` |
| 30 | `  •••start_col ≤ 25<br>	∧ •••start_line ≥ 252<br>	∧ -1.internal_type not in {CommentLine}<br>	∧ -1.reserved = (<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.length ≤ 4<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {)}<br>	∧ ^1.roles not in {FUNCTION, OPERATOR}<br>	∧ ^2.roles in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.948. Support: 183.` |
| 31 | `  •••start_col ≤ 25<br>	∧ •••start_line ≥ 252<br>	∧ -1.internal_type not in {CommentLine}<br>	∧ -1.reserved = (<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -3.reserved not in {(}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), function}<br>	∧ +1.roles not in {COMMENT}<br>	∧ +1.length ≤ 12<br>	∧ +2.reserved not in {.}<br>	∧ ^1.roles not in {FUNCTION, OPERATOR, QUALIFIED}<br>	∧ ^2.roles in {EXPRESSION} and not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.925. Support: 542.` |
| 32 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.reserved = }<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.roles not in {COMMENT}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type = IfStatement<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 223.` |
| 33 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.reserved = }<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.roles not in {COMMENT}<br>	∧ +1.length ≥ 2<br>	∧ +2.length ≥ 27<br>	∧ ^1.internal_type not in {IfStatement}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ⏎⏎<br>Confidence: 0.962. Support: 677.` |
| 34 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.reserved = }<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.diff_offset ≥ 10<br>	∧ +1.roles not in {COMMENT}<br>	∧ +1.length ≥ 2<br>	∧ +2.length ≤ 26<br>	∧ ^1.internal_type not in {IfStatement}<br>	∧ ^1.roles in {BODY} and not in {OPERATOR, QUALIFIED}<br>⇒ y = ⏎⏎<br>Confidence: 0.954. Support: 142.` |
| 35 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.reserved = }<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.label in {<-space>}<br>	∧ +1.reserved = }<br>	∧ +1.roles not in {COMMENT}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.992. Support: 330.` |
| 36 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.reserved = }<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.label not in {<-space>}<br>	∧ +1.reserved = }<br>	∧ +1.roles not in {COMMENT}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ⏎⇥⁻<br>Confidence: 0.980. Support: 530.` |
| 37 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.reserved = }<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = ,<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {FunctionExpression}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 502.` |
| 38 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.label in {<space>}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.reserved = (<br>	∧ +1.reserved not in {;}<br>	∧ +1.roles not in {COMMENT}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = "<br>Confidence: 0.997. Support: 192.` |
| 39 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.diff_offset ≥ 21<br>	∧ -2.roles not in {BLOCK}<br>	∧ +1.roles not in {COMMENT}<br>	∧ +1.length ≤ 2<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.958. Support: 998.` |
| 40 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.label in {<newline>} and not in {<space>}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.diff_offset ≤ 20<br>	∧ +1.reserved not in {;}<br>	∧ +1.roles in {KEY} and not in {COMMENT}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = "<br>Confidence: 0.920. Support: 94.` |
| 41 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.diff_offset ≤ 20<br>	∧ +1.reserved = ,<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 570.` |
| 42 | `  •••start_line ≥ 160<br>	∧ -1.diff_col ≥ 8<br>	∧ -1.internal_type not in {CommentLine}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.diff_offset ≤ 20<br>	∧ -3.reserved not in {:}<br>	∧ -4.diff_offset ≥ 6<br>	∧ +1.reserved not in {), ,, ;, }}<br>	∧ +1.roles in {FUNCTION} and not in {COMMENT, KEY}<br>	∧ +2.roles not in {ARGUMENT, COMMENT}<br>	∧ ^1.internal_type not in {VariableDeclaration}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED, SWITCH}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 513.` |
| 43 | `  •••start_line ≥ 160<br>	∧ -1.diff_col ≤ 7<br>	∧ -1.internal_type not in {CommentLine}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.diff_offset ≤ 20<br>	∧ -3.reserved not in {:}<br>	∧ -4.diff_offset ≥ 6<br>	∧ +1.reserved not in {), ,, ;, }}<br>	∧ +1.roles not in {COMMENT, KEY}<br>	∧ +2.roles not in {ARGUMENT, COMMENT}<br>	∧ ^1.internal_type not in {VariableDeclaration}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED, SWITCH}<br>⇒ y = ␣<br>Confidence: 0.973. Support: 19243.` |
| 44 | `  •••start_line ≤ 159<br>	∧ -1.internal_type not in {CommentLine}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.diff_offset ≤ 20<br>	∧ -2.label in {<space>} and not in {<newline>}<br>	∧ +1.internal_type not in {Identifier}<br>	∧ +1.reserved not in {,, ;, {, }}<br>	∧ +1.roles in {LITERAL} and not in {COMMENT, KEY}<br>	∧ +2.roles not in {COMMENT}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.947. Support: 159.` |
| 45 | `  •••start_line ≤ 159<br>	∧ -1.internal_type not in {CommentLine}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = )<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.diff_offset ≤ 20<br>	∧ -2.label not in {<newline>}<br>	∧ +1.internal_type not in {Identifier}<br>	∧ +1.reserved = =<br>	∧ +1.roles not in {COMMENT, KEY}<br>	∧ +2.roles not in {COMMENT}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.995. Support: 102.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 10.0, "max_conf": 0.9999529719352722, "max_support": 19243, "min_conf": 0.9202127456665039, "min_support": 90, "num_rules": 45}}
```
</details>
