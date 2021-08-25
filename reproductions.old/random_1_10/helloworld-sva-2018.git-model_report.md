# Model report for file:///tmp/top-repos-quality-repos-s3oe95oy/helloworld-sva-2018.git HEAD 940ab0451bc687b137896b0d9543f1c2a00b8916

### Dump

```json
{'created_at': '2021-08-20 12:11:10',
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
 'size': '20.4 kB',
 'tags': [],
 'uuid': 'e8889778-97ba-4030-8a58-a7bb1c602f5e',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-s3oe95oy/helloworld-sva-2018.git 940ab0451bc687b137896b0d9543f1c2a00b8916

# javascript
61 rules, avg.len. 8.9
## train
PPCR: 0.897389
### report
macro
{'f1-score': 0.7074180138717359,
 'precision': 0.7303232417736594,
 'recall': 0.6940933874910789,
 'support': 102437}
micro
{'f1-score': 0.9679998438064371,
 'precision': 0.9679998438064371,
 'recall': 0.9679998438064371,
 'support': 102437}
weighted
{'f1-score': 0.9652748769082554,
 'precision': 0.9631902001347599,
 'recall': 0.9679998438064371,
 'support': 102437}
### report_full
macro
{'f1-score': 0.5789733351947577,
 'precision': 0.7303232417736594,
 'recall': 0.5073454001786784,
 'support': 114150}
micro
{'f1-score': 0.9156505238079847,
 'precision': 0.9679998438064371,
 'recall': 0.8686727989487516,
 'support': 114150}
weighted
{'f1-score': 0.9058010743830399,
 'precision': 0.9557655592287528,
 'recall': 0.8686727989487516,
 'support': 114150}
## test
PPCR: 0.899951
### report
macro
{'f1-score': 0.5996108867034056,
 'precision': 0.616679414721237,
 'recall': 0.589587708922921,
 'support': 12899}
micro
{'f1-score': 0.9519342584696489,
 'precision': 0.9519342584696489,
 'recall': 0.9519342584696489,
 'support': 12899}
weighted
{'f1-score': 0.946767804658969,
 'precision': 0.942438985735837,
 'recall': 0.9519342584696489,
 'support': 12899}
### report_full
macro
{'f1-score': 0.49901757805234315,
 'precision': 0.616679414721237,
 'recall': 0.44908647129888835,
 'support': 14333}
micro
{'f1-score': 0.9018066980023501,
 'precision': 0.9519342584696489,
 'recall': 0.8566943417288774,
 'support': 14333}
weighted
{'f1-score': 0.8872024782546073,
 'precision': 0.9327454700971451,
 'recall': 0.8566943417288774,
 'support': 14333}
```

## javascript
### Summary
41 rules, avg.len. 8.6

| | |
|-|-|
|Min support|91|
|Max support|19592|
|Min confidence|0.9257095456123352|
|Max confidence|0.9995458722114563|

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
                     'min_samples_leaf': 91,
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
| 1 | `  ^1.roles in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 19592.` |
| 2 | `  +1.reserved = ]<br>	∧ ^1.roles in {OPERATOR} and not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 1.000. Support: 1101.` |
| 3 | `  -1.reserved = (<br>	∧ +1.reserved not in {]}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.957. Support: 995.` |
| 4 | `  -1.reserved not in {(}<br>	∧ -2.roles in {LEFT}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.964. Support: 913.` |
| 5 | `  -1.reserved not in {(}<br>	∧ +1.reserved = )<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.969. Support: 507.` |
| 6 | `  -1.label not in {<space>}<br>	∧ -1.reserved = {<br>	∧ -2.reserved not in {), [}<br>	∧ +1.reserved not in {)}<br>	∧ ^1.roles in {OPERATOR} and not in {QUALIFIED}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.985. Support: 166.` |
| 7 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -2.reserved not in {), [}<br>	∧ -2.roles not in {LEFT}<br>	∧ -3.reserved not in {=}<br>	∧ +1.reserved not in {)}<br>	∧ ^1.roles in {OPERATOR} and not in {QUALIFIED}<br>	∧ ^2.roles in {STATEMENT}<br>⇒ y = ␣<br>Confidence: 0.981. Support: 8955.` |
| 8 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {LEFT}<br>	∧ -2.label in {<space>}<br>	∧ -2.reserved not in {), [}<br>	∧ -2.roles not in {LEFT}<br>	∧ +1.reserved not in {)}<br>	∧ ^1.roles in {OPERATOR} and not in {QUALIFIED}<br>	∧ ^2.roles not in {STATEMENT}<br>⇒ y = ␣<br>Confidence: 0.974. Support: 3078.` |
| 9 | `  -1.reserved = ;<br>	∧ +1.reserved not in {}}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>	∧ ^2.roles in {SCOPE}<br>⇒ y = ␣<br>Confidence: 0.958. Support: 793.` |
| 10 | `  -1.reserved = ;<br>	∧ +1.reserved not in {}}<br>	∧ +2.reserved = .<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>	∧ ^2.roles not in {SCOPE}<br>⇒ y = ⏎<br>Confidence: 0.955. Support: 1055.` |
| 11 | `  -1.reserved not in {;}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ -3.reserved = var<br>	∧ +1.reserved = =<br>	∧ +3.reserved not in {,}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.932. Support: 1354.` |
| 12 | `  -1.reserved not in {;}<br>	∧ -1.roles in {EXPRESSION} and not in {STRING}<br>	∧ -3.reserved not in {var}<br>	∧ ^1.internal_type = CallExpression<br>	∧ ^1.roles in {CONDITION} and not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.996. Support: 134.` |
| 13 | `  -1.reserved not in {;}<br>	∧ -1.roles in {EXPRESSION} and not in {STRING}<br>	∧ -3.reserved not in {var}<br>	∧ ^1.roles not in {CONDITION, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.980. Support: 13501.` |
| 14 | `  -1.reserved = (<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type = StringLiteral<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = '<br>Confidence: 0.985. Support: 165.` |
| 15 | `  -1.reserved = (<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.980. Support: 3858.` |
| 16 | `  -1.reserved not in {(, ;}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = ;<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 3110.` |
| 17 | `  -1.reserved = {<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +3.roles not in {BINARY}<br>	∧ ^1.internal_type not in {ForStatement}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>	∧ ^2.roles in {BINARY}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.978. Support: 346.` |
| 18 | `  -1.internal_type = CommentLine<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {;}<br>	∧ ^1.internal_type = File<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ⏎<br>Confidence: 0.926. Support: 599.` |
| 19 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.length ≥ 44<br>	∧ +1.reserved not in {;}<br>	∧ ^1.internal_type = File<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ⏎<br>Confidence: 0.945. Support: 373.` |
| 20 | `  -1.diff_col ≥ 2<br>	∧ -1.internal_type not in {CommentLine}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {;}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.internal_type = File<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.927. Support: 157.` |
| 21 | `  -1.diff_col ≥ 2<br>	∧ -1.internal_type not in {CommentLine}<br>	∧ -1.reserved not in {;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {(}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.internal_type = File<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.977. Support: 2027.` |
| 22 | `  -1.diff_col ≤ 1<br>	∧ -1.internal_type not in {CommentLine}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -3.roles in {BLOCK}<br>	∧ +1.reserved not in {;}<br>	∧ ^1.internal_type = File<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ⏎<br>Confidence: 0.998. Support: 205.` |
| 23 | `  -1.diff_col ≤ 1<br>	∧ -1.internal_type not in {CommentLine}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = {<br>	∧ ^1.internal_type = File<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.985. Support: 97.` |
| 24 | `  -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = )<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.934. Support: 679.` |
| 25 | `  -1.reserved = }<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = else<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.976. Support: 232.` |
| 26 | `  -1.reserved = }<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.label in {<-space>}<br>	∧ +1.reserved = }<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ⏎␣⁻␣⁻␣⁻␣⁻<br>Confidence: 0.996. Support: 115.` |
| 27 | `  -1.reserved = }<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.label not in {<-space>}<br>	∧ +1.reserved = }<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.983. Support: 202.` |
| 28 | `  -1.reserved = ,<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -4.diff_col ≥ 25<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles in {LITERAL} and not in {OPERATOR}<br>	∧ ^2.roles in {INSTANCE}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 206.` |
| 29 | `  •••start_col ≥ 14<br>	∧ -1.reserved = ,<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -4.diff_col ≤ 24<br>	∧ +3.roles in {COMMENT}<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles in {LITERAL} and not in {OPERATOR}<br>	∧ ^2.roles in {INSTANCE}<br>⇒ y = ∅<br>Confidence: 0.996. Support: 112.` |
| 30 | `  •••start_col ≥ 14<br>	∧ -1.reserved = ,<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -4.diff_col ≤ 24<br>	∧ +4.roles in {COMMENT}<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles in {LITERAL} and not in {OPERATOR}<br>	∧ ^2.roles in {INSTANCE}<br>⇒ y = ∅<br>Confidence: 0.995. Support: 91.` |
| 31 | `  •••start_col ≥ 14<br>	∧ -1.reserved = ,<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -4.diff_col ≤ 24<br>	∧ +1.reserved not in {)}<br>	∧ +3.roles not in {COMMENT}<br>	∧ +4.roles not in {COMMENT}<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles in {LITERAL} and not in {OPERATOR, QUALIFIED}<br>	∧ ^2.roles in {INSTANCE}<br>⇒ y = ␣<br>Confidence: 0.962. Support: 93.` |
| 32 | `  •••start_col ≤ 13<br>	∧ -1.reserved = ,<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -4.diff_col ≤ 24<br>	∧ +1.reserved not in {)}<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles in {LITERAL} and not in {OPERATOR, QUALIFIED}<br>	∧ ^2.roles in {INSTANCE}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 164.` |
| 33 | `  -1.reserved not in {(, ,, ;, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles in {LITERAL} and not in {OPERATOR}<br>	∧ ^2.roles in {INSTANCE}<br>⇒ y = ∅<br>Confidence: 0.992. Support: 323.` |
| 34 | `  -1.reserved not in {(, ;, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -3.reserved = ,<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.942. Support: 474.` |
| 35 | `  -1.reserved = ++<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.roles not in {INITIALIZATION}<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 174.` |
| 36 | `  -1.reserved = ]<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.roles not in {INITIALIZATION}<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.971. Support: 155.` |
| 37 | `  -1.diff_col ≤ 6<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {++, ;, ], {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.length ≤ 20<br>	∧ -5.label in {<newline>}<br>	∧ ^1.internal_type = VariableDeclarator<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.996. Support: 131.` |
| 38 | `  -1.diff_col ≤ 6<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, ], {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.length ≤ 20<br>	∧ -3.diff_col ≥ 4<br>	∧ -3.reserved not in {,}<br>	∧ -5.label in {<newline>}<br>	∧ +1.reserved not in {), ;}<br>	∧ ^1.internal_type not in {File, VariableDeclarator}<br>	∧ ^1.roles not in {LITERAL, OPERATOR, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.955. Support: 301.` |
| 39 | `  •••start_col ≥ 7<br>	∧ -1.diff_col ≤ 6<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, ], {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.length ≤ 20<br>	∧ -3.reserved not in {,}<br>	∧ -5.diff_offset ≥ 11<br>	∧ -5.label not in {<newline>}<br>	∧ +1.reserved not in {(, ), ;}<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles not in {LITERAL, OPERATOR, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.961. Support: 5133.` |
| 40 | `  •••start_col ≥ 7<br>	∧ -1.diff_col ≤ 6<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, ], {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.length ≤ 20<br>	∧ -3.reserved not in {,}<br>	∧ -3.roles in {EXPRESSION}<br>	∧ -5.diff_offset ≤ 10<br>	∧ -5.label not in {<newline>}<br>	∧ +1.reserved not in {(, ), ;}<br>	∧ +1.roles in {NUMBER}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles not in {LITERAL, OPERATOR, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.990. Support: 150.` |
| 41 | `  •••start_col ≥ 7<br>	∧ -1.diff_col ≤ 6<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, ], {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.length ≤ 20<br>	∧ -3.reserved not in {,}<br>	∧ -5.diff_offset ≤ 10<br>	∧ -5.label not in {<newline>}<br>	∧ +1.reserved not in {(, ), ;}<br>	∧ +1.roles not in {NUMBER}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles not in {LITERAL, OPERATOR, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.926. Support: 2272.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 8.609756097560975, "max_conf": 0.9995458722114563, "max_support": 19592, "min_conf": 0.9257095456123352, "min_support": 91, "num_rules": 41}}
```
</details>
