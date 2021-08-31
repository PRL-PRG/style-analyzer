# Model report for file:///tmp/top-repos-quality-repos-ryqlbjzk/content.git HEAD 0b6a7be6f6287cde880b0d37c88e6461efa167ff

### Dump

```json
{'created_at': '2021-08-30 17:20:36',
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
 'size': '18.4 kB',
 'tags': [],
 'uuid': 'df1a7b66-05e2-4fff-b1bd-b81d3e68136b',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-ryqlbjzk/content.git 0b6a7be6f6287cde880b0d37c88e6461efa167ff

# javascript
131 rules, avg.len. 7.3
## train
PPCR: 0.971249
### report
macro
{'f1-score': 0.8640776394712623,
 'precision': 0.8886616014980532,
 'recall': 0.8477760131812689,
 'support': 21890}
micro
{'f1-score': 0.9180447693010507,
 'precision': 0.9180447693010507,
 'recall': 0.9180447693010507,
 'support': 21890}
weighted
{'f1-score': 0.9167157372226971,
 'precision': 0.9172930114032176,
 'recall': 0.9180447693010507,
 'support': 21890}
### report_full
macro
{'f1-score': 0.8191464084053447,
 'precision': 0.8886616014980532,
 'recall': 0.7848910136582282,
 'support': 22538}
micro
{'f1-score': 0.9046547222472314,
 'precision': 0.9180447693010507,
 'recall': 0.8916496583547786,
 'support': 22538}
weighted
{'f1-score': 0.8991183226000833,
 'precision': 0.9153293154369723,
 'recall': 0.8916496583547786,
 'support': 22538}
## test
PPCR: 0.974453
### report
macro
{'f1-score': 0.7744828717849694,
 'precision': 0.785457697617501,
 'recall': 0.7776339939645488,
 'support': 3967}
micro
{'f1-score': 0.8784976052432568,
 'precision': 0.8784976052432568,
 'recall': 0.8784976052432568,
 'support': 3967}
weighted
{'f1-score': 0.8754353227023597,
 'precision': 0.8773076313242293,
 'recall': 0.8784976052432568,
 'support': 3967}
### report_full
macro
{'f1-score': 0.7461041669913995,
 'precision': 0.785457697617501,
 'recall': 0.738312207897288,
 'support': 4071}
micro
{'f1-score': 0.8671311271460562,
 'precision': 0.8784976052432568,
 'recall': 0.8560550233357898,
 'support': 4071}
weighted
{'f1-score': 0.8588100571168007,
 'precision': 0.8711503773411658,
 'recall': 0.8560550233357898,
 'support': 4071}
```

## javascript
### Summary
91 rules, avg.len. 6.7

| | |
|-|-|
|Min support|156|
|Max support|2997|
|Min confidence|0.920192301273346|
|Max confidence|0.9995567202568054|

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
| 1 | `  -1.internal_type = StringLiteral<br>⇒ y = '<br>Confidence: 1.000. Support: 1128.` |
| 2 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = :<br>	∧ +1.internal_type = StringLiteral<br>⇒ y = ␣<br>Confidence: 0.999. Support: 408.` |
| 3 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :}<br>	∧ +1.internal_type = StringLiteral<br>⇒ y = '<br>Confidence: 0.929. Support: 1103.` |
| 4 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.roles in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.980. Support: 2765.` |
| 5 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 619.` |
| 6 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles in {MAP}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {QUALIFIED}<br>	∧ ^2.roles in {INITIALIZATION}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.950. Support: 291.` |
| 7 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {MAP}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.972. Support: 337.` |
| 8 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = )<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles in {CALL}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {LITERAL, QUALIFIED}<br>⇒ y = ⏎⏎<br>Confidence: 0.981. Support: 183.` |
| 9 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = =<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 747.` |
| 10 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = {<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 170.` |
| 11 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = {<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.964. Support: 619.` |
| 12 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, ], {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ +4.reserved not in {>}<br>	∧ ^1.roles not in {DECLARATION, IF, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.990. Support: 2610.` |
| 13 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = )<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ +4.reserved not in {>}<br>	∧ ^1.roles not in {DECLARATION, IF, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.992. Support: 791.` |
| 14 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), =, ], {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ +4.reserved not in {>}<br>	∧ ^1.roles in {EXPRESSION} and not in {DECLARATION, IF, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.924. Support: 543.` |
| 15 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.roles in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.980. Support: 2809.` |
| 16 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles in {MAP}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {IDENTIFIER}<br>	∧ ^2.roles in {INITIALIZATION}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.964. Support: 263.` |
| 17 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {MAP}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.976. Support: 354.` |
| 18 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.992. Support: 589.` |
| 19 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = )<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ +3.roles in {CALL}<br>	∧ ^1.roles not in {IDENTIFIER, LITERAL}<br>⇒ y = ⏎⏎<br>Confidence: 0.997. Support: 183.` |
| 20 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ), {}<br>	∧ -3.diff_offset ≥ 5<br>	∧ -4.reserved not in {.}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {BLOCK, FILE, IDENTIFIER, LITERAL}<br>⇒ y = ␣<br>Confidence: 0.963. Support: 1378.` |
| 21 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = =<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 739.` |
| 22 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = {<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.990. Support: 156.` |
| 23 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = {<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.956. Support: 669.` |
| 24 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.965. Support: 2963.` |
| 25 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≥ 2<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.966. Support: 252.` |
| 26 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≤ 1<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = )<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.993. Support: 777.` |
| 27 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≤ 1<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = ,<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles in {EXPRESSION} and not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 238.` |
| 28 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.internal_type = MemberExpression<br>⇒ y = ∅<br>Confidence: 0.985. Support: 2706.` |
| 29 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles in {MAP}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^2.roles in {INITIALIZATION}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.950. Support: 289.` |
| 30 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {MAP}<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.959. Support: 400.` |
| 31 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ∅<br>Confidence: 0.989. Support: 594.` |
| 32 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = )<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +1.length ≥ 2<br>	∧ +3.roles in {ARGUMENT}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {LITERAL}<br>⇒ y = ⏎⏎<br>Confidence: 0.997. Support: 191.` |
| 33 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {{}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = =<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 715.` |
| 34 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = {<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ∅<br>Confidence: 0.984. Support: 159.` |
| 35 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = {<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ␣<br>Confidence: 0.956. Support: 626.` |
| 36 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {{}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ∅<br>Confidence: 0.962. Support: 2997.` |
| 37 | `  -1.diff_col ≥ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {{}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ␣<br>Confidence: 0.979. Support: 262.` |
| 38 | `  -1.diff_col ≤ 1<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = )<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ∅<br>Confidence: 0.996. Support: 807.` |
| 39 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.953. Support: 2990.` |
| 40 | `  -1.diff_offset ≥ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.963. Support: 286.` |
| 41 | `  -1.diff_offset ≤ 1<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = )<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.987. Support: 834.` |
| 42 | `  -1.diff_offset ≤ 1<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = ,<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles in {EXPRESSION} and not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 232.` |
| 43 | `  -1.diff_offset ≤ 1<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = ><br>	∧ +1.length ≤ 1<br>	∧ ^1.roles in {EXPRESSION} and not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 218.` |
| 44 | `  -1.diff_offset ≤ 1<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), ,, =, >, {, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles in {EXPRESSION} and not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 172.` |
| 45 | `  -1.internal_type not in {StringLiteral}<br>	∧ ^1.internal_type = MemberExpression<br>⇒ y = ∅<br>Confidence: 0.983. Support: 2869.` |
| 46 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ +1.roles in {MAP}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^2.roles in {INITIALIZATION}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.936. Support: 274.` |
| 47 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ +1.roles not in {MAP}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.947. Support: 386.` |
| 48 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = :<br>	∧ +1.internal_type = StringLiteral<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 359.` |
| 49 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, {}<br>	∧ +1.internal_type = StringLiteral<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = '<br>Confidence: 0.953. Support: 1066.` |
| 50 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ∅<br>Confidence: 0.989. Support: 601.` |
| 51 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = )<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ +3.roles in {CALL}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {LITERAL}<br>⇒ y = ⏎⏎<br>Confidence: 0.993. Support: 209.` |
| 52 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = =<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 707.` |
| 53 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ +1.reserved = {<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ∅<br>Confidence: 0.985. Support: 162.` |
| 54 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(}<br>	∧ +1.reserved = {<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ␣<br>Confidence: 0.961. Support: 624.` |
| 55 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ +4.reserved not in {>}<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ∅<br>Confidence: 0.961. Support: 2899.` |
| 56 | `  -1.internal_type not in {StringLiteral}<br>	∧ ^1.roles in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.981. Support: 2758.` |
| 57 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = :<br>	∧ +1.internal_type = StringLiteral<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 372.` |
| 58 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :}<br>	∧ +1.internal_type = StringLiteral<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = '<br>Confidence: 0.948. Support: 1063.` |
| 59 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = )<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ +3.roles in {CALL}<br>	∧ ^1.roles not in {LITERAL, QUALIFIED}<br>⇒ y = ⏎⏎<br>Confidence: 0.986. Support: 175.` |
| 60 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = =<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 719.` |
| 61 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ +1.reserved = {<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.991. Support: 159.` |
| 62 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(}<br>	∧ +1.reserved = {<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.965. Support: 611.` |
| 63 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles not in {LITERAL}<br>	∧ +1.length ≤ 1<br>	∧ +4.reserved not in {>}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.956. Support: 2977.` |
| 64 | `  -1.diff_offset ≥ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.roles not in {LITERAL}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.929. Support: 163.` |
| 65 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ +1.roles in {MAP}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {QUALIFIED}<br>	∧ ^2.roles in {INITIALIZATION}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.936. Support: 289.` |
| 66 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ +1.roles not in {MAP}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.958. Support: 391.` |
| 67 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, {}<br>	∧ +1.internal_type = StringLiteral<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = '<br>Confidence: 0.956. Support: 1074.` |
| 68 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ), {}<br>	∧ -3.diff_offset ≥ 5<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {BLOCK, FILE, LITERAL, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.924. Support: 1518.` |
| 69 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.958. Support: 2956.` |
| 70 | `  -1.diff_col ≥ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.963. Support: 283.` |
| 71 | `  -1.diff_col ≤ 1<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = )<br>	∧ +1.roles not in {LITERAL}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.994. Support: 772.` |
| 72 | `  -1.internal_type not in {StringLiteral}<br>	∧ ^1.roles in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.983. Support: 2973.` |
| 73 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = :<br>	∧ +1.internal_type = StringLiteral<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 386.` |
| 74 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :}<br>	∧ +1.internal_type = StringLiteral<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = '<br>Confidence: 0.946. Support: 1087.` |
| 75 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = =<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 718.` |
| 76 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ +1.reserved = {<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.980. Support: 171.` |
| 77 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(}<br>	∧ +1.reserved = {<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.970. Support: 618.` |
| 78 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.964. Support: 2869.` |
| 79 | `  -1.diff_offset ≥ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.979. Support: 305.` |
| 80 | `  -1.diff_offset ≤ 1<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = )<br>	∧ +1.roles not in {LITERAL}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.988. Support: 817.` |
| 81 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles in {MAP}<br>	∧ ^1.roles not in {QUALIFIED}<br>	∧ ^2.roles in {INITIALIZATION}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.944. Support: 278.` |
| 82 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {MAP}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.955. Support: 390.` |
| 83 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 627.` |
| 84 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = )<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +1.length ≥ 2<br>	∧ +3.roles in {CALL}<br>	∧ ^1.roles not in {LITERAL, QUALIFIED}<br>⇒ y = ⏎⏎<br>Confidence: 0.992. Support: 186.` |
| 85 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {{}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = =<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 760.` |
| 86 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = {<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.965. Support: 617.` |
| 87 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {{}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, ], {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ +4.reserved not in {>}<br>	∧ ^1.roles not in {BINARY, DECLARATION, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.991. Support: 2575.` |
| 88 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = )<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ +4.reserved not in {>}<br>	∧ ^1.roles not in {BINARY, DECLARATION, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.989. Support: 831.` |
| 89 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {{}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), =, ], {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ +4.reserved not in {>}<br>	∧ ^1.roles in {EXPRESSION} and not in {BINARY, DECLARATION, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.920. Support: 520.` |
| 90 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {{}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.952. Support: 2928.` |
| 91 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {{}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≥ 2<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.960. Support: 291.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 6.6923076923076925, "max_conf": 0.9995567202568054, "max_support": 2997, "min_conf": 0.920192301273346, "min_support": 156, "num_rules": 91}}
```
</details>
