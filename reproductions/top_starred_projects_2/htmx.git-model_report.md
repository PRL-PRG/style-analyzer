# Model report for file:///tmp/top-repos-quality-repos-cu3ifep7/htmx.git HEAD 7e7a7100b8557ea36538c3efdbd5b1c9129f0fc9

### Dump

```json
{'created_at': '2021-08-30 06:48:52',
 'datasets': [],
 'dependencies': [],
 'description': 'Model bound to style.format.analyzer.FormatAnalyzer Lookout analyzer.',
 'environment': {'packages': 'ConfigArgParse==0.13.0 Jinja2==2.10 MarkupSafe==1.1.1 PyStemmer==1.3.0 PyYAML==5.1 Pympler==0.5 SQLAlchemy==1.2.10 SQLAlchemy-Utils==0.33.3 asdf==2.3.2 bblfsh==2.12.7 boto==2.49.0 boto3==1.9.130 botocore==1.12.130 cachetools==2.0.1 certifi==2019.3.9 chardet==3.0.4 clint==0.5.1 docker==3.7.0 docker-pycreds==0.4.0 dulwich==0.19.11 grpcio==1.19.0 grpcio-tools==1.19.0 humanfriendly==4.16.1 humanize==0.5.1 idna==2.8 jmespath==0.9.4 jsonschema==2.6.0 lookout-sdk==0.4.1 lookout-sdk-ml==0.19.0 lookout-style==0.2.0 lz4==2.1.6 modelforge==0.12.1 numpy==1.16.2 packaging==19.0 pandas==0.22.0 pip==19.0.3 protobuf==3.7.0 psycopg2-binary==2.7.5 pygtrie==2.3 pyparsing==2.3.1 python-dateutil==2.8.0 python-igraph==0.7.1.post6 pytz==2019.1 requests==2.21.0 requirements-parser==0.2.0 scikit-learn==0.20.1 scikit-optimize==0.5.2 scipy==1.2.1 semantic-version==2.6.0 setuptools==40.8.0 six==1.12.0 smart-open==1.8.1 sourced-ml==0.8.2 spdx==2.5.0 stringcase==1.2.0 tabulate==0.8.2 tqdm==4.31.1 '
                             'urllib3==1.24.1 websocket-client==0.55.0 xxhash==1.3.0',
                 'platform': 'Linux-5.11.0-31-generic-x86_64-with-Ubuntu-18.04-bionic',
                 'python': '3.6.7 (default, Oct 22 2018, 11:32:17) [GCC 8.2.0]'},
 'license': 'ODbL-1.0',
 'metrics': {},
 'model': 'style.format.analyzer.FormatAnalyzer',
 'references': [],
 'series': 'Lookout',
 'size': '31.0 kB',
 'tags': [],
 'uuid': '0f81bd13-7ccd-4bee-a09f-cd19d59ae509',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-cu3ifep7/htmx.git 7e7a7100b8557ea36538c3efdbd5b1c9129f0fc9

# javascript
195 rules, avg.len. 10.4
## train
PPCR: 0.964516
### report
macro
{'f1-score': 0.4642997301307006,
 'precision': 0.47450250843346364,
 'recall': 0.45612838979665826,
 'support': 635013}
micro
{'f1-score': 0.9677108972572215,
 'precision': 0.9677108972572215,
 'recall': 0.9677108972572215,
 'support': 635013}
weighted
{'f1-score': 0.9662515426632534,
 'precision': 0.965764539976603,
 'recall': 0.9677108972572215,
 'support': 635013}
### report_full
macro
{'f1-score': 0.44197061668012383,
 'precision': 0.47450250843346364,
 'recall': 0.41965475966903,
 'support': 658375}
micro
{'f1-score': 0.9502314850609408,
 'precision': 0.9677108972572215,
 'recall': 0.9333723182077084,
 'support': 658375}
weighted
{'f1-score': 0.9464086493329799,
 'precision': 0.9636914796754196,
 'recall': 0.9333723182077084,
 'support': 658375}
## test
PPCR: 0.961160
### report
macro
{'f1-score': 0.45939038865526394,
 'precision': 0.46995415189868206,
 'recall': 0.45120442009165107,
 'support': 155185}
micro
{'f1-score': 0.9637851596481619,
 'precision': 0.9637851596481619,
 'recall': 0.9637851596481619,
 'support': 155185}
weighted
{'f1-score': 0.9621926714130712,
 'precision': 0.9617826373216596,
 'recall': 0.9637851596481619,
 'support': 155185}
### report_full
macro
{'f1-score': 0.4345305249743766,
 'precision': 0.46995415189868206,
 'recall': 0.41088353571051234,
 'support': 161456}
micro
{'f1-score': 0.9446976228599581,
 'precision': 0.9637851596481619,
 'recall': 0.9263514517887226,
 'support': 161456}
weighted
{'f1-score': 0.9402483980320091,
 'precision': 0.9594570864062152,
 'recall': 0.9263514517887226,
 'support': 161456}
```

## javascript
### Summary
122 rules, avg.len. 10.3

| | |
|-|-|
|Min support|90|
|Max support|55282|
|Min confidence|0.9207746386528015|
|Max confidence|0.9997459650039673|

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
                     'min_samples_split': 181,
                     'n_estimators': 10,
                     'prune_attributes': True,
                     'prune_branches_algorithms': ['reduced-error'],
                     'prune_dataset_ratio': 0.2,
                     'top_down_greedy_budget': [False, 0.5]}}
```

### Rules

| rule number | description |
|----:|:-----|
| 1 | `  •••start_col ≥ 26<br>	∧ -1.roles in {STRING}<br>	∧ -2.reserved = [<br>	∧ -3.length ≥ 8<br>	∧ ^1.roles in {IDENTIFIER}<br>⇒ y = '<br>Confidence: 0.983. Support: 90.` |
| 2 | `  •••start_col ≥ 26<br>	∧ -1.roles in {STRING}<br>	∧ -2.reserved = [<br>	∧ -3.length ≤ 8<br>	∧ ^1.roles in {IDENTIFIER}<br>⇒ y = "<br>Confidence: 0.995. Support: 99.` |
| 3 | `  •••start_col ≤ 26<br>	∧ -1.roles in {STRING}<br>	∧ -2.reserved = [<br>	∧ ^1.roles in {IDENTIFIER}<br>⇒ y = '<br>Confidence: 0.999. Support: 373.` |
| 4 | `  -1.roles in {STRING}<br>	∧ -2.reserved not in {[}<br>	∧ ^1.roles in {IDENTIFIER}<br>⇒ y = "<br>Confidence: 0.952. Support: 2147.` |
| 5 | `  •••start_line ≤ 228<br>	∧ -1.roles not in {STRING}<br>	∧ -2.diff_col ≥ 12<br>	∧ -4.diff_offset ≥ 10<br>	∧ +1.internal_type = StringLiteral<br>	∧ +1.length ≥ 7<br>	∧ ^1.roles in {IDENTIFIER}<br>⇒ y = '<br>Confidence: 0.996. Support: 113.` |
| 6 | `  •••start_line ≤ 228<br>	∧ -1.roles not in {STRING}<br>	∧ -4.diff_offset ≥ 10<br>	∧ +1.internal_type = StringLiteral<br>	∧ +1.length ≤ 6<br>	∧ ^1.roles in {IDENTIFIER}<br>⇒ y = '<br>Confidence: 0.988. Support: 612.` |
| 7 | `  -1.roles not in {STRING}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ ^1.roles in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 1.000. Support: 38782.` |
| 8 | `  -1.reserved = ;<br>	∧ +1.length ≥ 2<br>	∧ +3.length ≥ 17<br>	∧ +5.length ≥ 6<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ⏎⏎<br>Confidence: 0.954. Support: 2215.` |
| 9 | `  -1.reserved = ;<br>	∧ -3.reserved = (<br>	∧ +1.length ≥ 2<br>	∧ +3.length ≤ 16<br>	∧ ^1.roles not in {FILE, IDENTIFIER}<br>⇒ y = ⏎<br>Confidence: 0.989. Support: 6822.` |
| 10 | `  •••start_col ≥ 8<br>	∧ -1.reserved = ;<br>	∧ -3.reserved not in {(}<br>	∧ -4.diff_offset ≥ 7<br>	∧ -4.roles not in {BINARY}<br>	∧ +1.length ≥ 2<br>	∧ +3.roles in {ARGUMENT}<br>	∧ +3.length ≤ 16<br>	∧ ^1.roles not in {FILE, IDENTIFIER}<br>⇒ y = ⏎<br>Confidence: 0.924. Support: 615.` |
| 11 | `  •••start_col ≥ 8<br>	∧ -1.reserved = ;<br>	∧ -3.reserved not in {(}<br>	∧ -4.reserved = *<br>	∧ -4.roles not in {BINARY}<br>	∧ +1.length ≥ 2<br>	∧ +3.roles not in {ARGUMENT}<br>	∧ +3.length ≤ 3<br>	∧ ^1.roles not in {FILE, IDENTIFIER}<br>⇒ y = ⏎⏎<br>Confidence: 0.995. Support: 95.` |
| 12 | `  •••start_col ≥ 8<br>	∧ -1.reserved = ;<br>	∧ -3.reserved not in {(}<br>	∧ -4.reserved not in {*}<br>	∧ -5.diff_offset ≥ 14<br>	∧ +1.length ≥ 2<br>	∧ +3.roles not in {ARGUMENT}<br>	∧ +3.length ≤ 16<br>	∧ +5.reserved = (<br>	∧ ^1.roles not in {FILE, IDENTIFIER}<br>	∧ ^2.internal_type not in {BlockStatement}<br>⇒ y = ⏎<br>Confidence: 0.927. Support: 1096.` |
| 13 | `  •••start_col ≥ 8<br>	∧ -1.reserved = ;<br>	∧ -3.reserved not in {(}<br>	∧ -4.reserved not in {*}<br>	∧ -5.diff_col ≥ 12<br>	∧ -5.diff_offset ≤ 13<br>	∧ -5.reserved = )<br>	∧ -5.roles not in {EXPRESSION}<br>	∧ +1.length ≥ 2<br>	∧ +3.roles not in {ARGUMENT}<br>	∧ +3.length ≤ 16<br>	∧ +5.reserved = (<br>	∧ ^1.roles not in {FILE, IDENTIFIER}<br>	∧ ^2.internal_type not in {BlockStatement}<br>⇒ y = ⏎<br>Confidence: 0.997. Support: 183.` |
| 14 | `  •••start_col ≥ 8<br>	∧ -1.reserved = ;<br>	∧ -3.reserved not in {(}<br>	∧ -4.reserved not in {*}<br>	∧ -5.diff_col ≤ 11<br>	∧ -5.diff_offset ≤ 13<br>	∧ -5.roles not in {EXPRESSION}<br>	∧ +1.length ≥ 2<br>	∧ +3.roles not in {ARGUMENT}<br>	∧ +3.length ≤ 16<br>	∧ +5.reserved = (<br>	∧ ^1.roles not in {FILE, IDENTIFIER}<br>	∧ ^2.internal_type not in {BlockStatement}<br>⇒ y = ⏎<br>Confidence: 0.961. Support: 369.` |
| 15 | `  •••start_col ≥ 8<br>	∧ •••start_line ≤ 134<br>	∧ -1.reserved = ;<br>	∧ -3.diff_col ≤ 14<br>	∧ -4.diff_col ≥ 13<br>	∧ -4.internal_type not in {Identifier}<br>	∧ -4.reserved not in {*}<br>	∧ +1.length ≥ 2<br>	∧ +3.roles not in {ARGUMENT}<br>	∧ +3.length ≤ 16<br>	∧ +5.reserved = )<br>	∧ ^1.roles not in {FILE, IDENTIFIER}<br>	∧ ^2.internal_type not in {BlockStatement}<br>⇒ y = ⏎<br>Confidence: 0.999. Support: 845.` |
| 16 | `  •••start_col ≥ 8<br>	∧ •••start_line ≤ 134<br>	∧ -1.reserved = ;<br>	∧ -3.diff_col ≤ 14<br>	∧ -3.reserved not in {(}<br>	∧ -4.internal_type not in {Identifier}<br>	∧ -4.reserved not in {*}<br>	∧ +1.length ≥ 2<br>	∧ +3.roles not in {ARGUMENT}<br>	∧ +3.length ≤ 16<br>	∧ +5.reserved not in {(, )}<br>	∧ ^1.roles not in {FILE, IDENTIFIER}<br>	∧ ^2.internal_type not in {BlockStatement}<br>⇒ y = ⏎<br>Confidence: 0.977. Support: 4370.` |
| 17 | `  -1.reserved = (<br>	∧ -2.diff_offset ≥ 6<br>	∧ +1.internal_type = StringLiteral<br>	∧ +1.length ≥ 19<br>	∧ +3.length ≥ 3<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = "<br>Confidence: 0.953. Support: 354.` |
| 18 | `  •••start_col ≥ 28<br>	∧ -1.reserved = (<br>	∧ -2.diff_offset ≤ 5<br>	∧ +1.internal_type = StringLiteral<br>	∧ +1.length ≥ 19<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = "<br>Confidence: 0.921. Support: 284.` |
| 19 | `  •••start_col ≤ 27<br>	∧ -1.reserved = (<br>	∧ -2.diff_offset ≤ 5<br>	∧ -5.label in {<newline>}<br>	∧ +1.internal_type = StringLiteral<br>	∧ +1.length ≥ 19<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = "<br>Confidence: 0.986. Support: 184.` |
| 20 | `  •••start_col ≤ 27<br>	∧ -1.reserved = (<br>	∧ -2.diff_offset ≤ 5<br>	∧ -5.label not in {<newline>}<br>	∧ +1.internal_type = StringLiteral<br>	∧ +1.length ≥ 19<br>	∧ +5.length ≥ 2<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = '<br>Confidence: 0.944. Support: 1430.` |
| 21 | `  •••start_col ≤ 27<br>	∧ •••start_line ≥ 118<br>	∧ -1.reserved = (<br>	∧ -2.diff_offset ≤ 5<br>	∧ -5.label not in {<newline>}<br>	∧ -5.reserved not in {}}<br>	∧ +1.internal_type = StringLiteral<br>	∧ +1.length ≤ 48<br>	∧ +5.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = '<br>Confidence: 0.939. Support: 221.` |
| 22 | `  •••start_col ≤ 27<br>	∧ -1.reserved = (<br>	∧ -2.diff_offset ≤ 5<br>	∧ -5.label not in {<newline>}<br>	∧ +1.internal_type = StringLiteral<br>	∧ +1.length ≤ 34<br>	∧ +5.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = '<br>Confidence: 0.923. Support: 1437.` |
| 23 | `  -1.reserved = (<br>	∧ -2.diff_col ≥ 3<br>	∧ -3.label in {<space>}<br>	∧ +1.internal_type = StringLiteral<br>	∧ +1.length ≤ 13<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = "<br>Confidence: 0.939. Support: 255.` |
| 24 | `  -1.reserved = (<br>	∧ -2.diff_col ≥ 3<br>	∧ -3.label in {<space>}<br>	∧ +1.internal_type = StringLiteral<br>	∧ +1.length ≤ 4<br>	∧ +4.reserved = .<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = "<br>Confidence: 0.968. Support: 111.` |
| 25 | `  •••start_col ≤ 34<br>	∧ -1.reserved = (<br>	∧ -2.diff_col ≥ 3<br>	∧ -3.label not in {<space>}<br>	∧ +1.internal_type = StringLiteral<br>	∧ +1.length ≤ 18<br>	∧ +3.reserved = ;<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = '<br>Confidence: 0.936. Support: 211.` |
| 26 | `  -1.reserved = (<br>	∧ -2.diff_col ≥ 3<br>	∧ -3.label not in {<space>}<br>	∧ +1.internal_type = StringLiteral<br>	∧ +1.length ≤ 18<br>	∧ +3.reserved not in {{}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = "<br>Confidence: 0.953. Support: 7169.` |
| 27 | `  -1.reserved = (<br>	∧ -2.diff_col ≤ 2<br>	∧ +1.internal_type = StringLiteral<br>	∧ +1.length ≤ 13<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = "<br>Confidence: 0.990. Support: 260.` |
| 28 | `  -1.reserved = (<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 20677.` |
| 29 | `  •••start_col ≤ 49<br>	∧ -1.reserved = {<br>	∧ -2.label in {<space>}<br>	∧ +1.roles in {MAP}<br>	∧ +1.length ≥ 2<br>	∧ +5.internal_type = Identifier<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ⏎␣⁺␣⁺␣⁺␣⁺<br>Confidence: 0.953. Support: 647.` |
| 30 | `  •••start_line ≥ 2<br>	∧ -1.reserved = {<br>	∧ +1.roles not in {MAP}<br>	∧ +1.length ≥ 2<br>	∧ +3.reserved not in {}}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ⏎␣⁺␣⁺␣⁺␣⁺<br>Confidence: 0.970. Support: 14647.` |
| 31 | `  •••start_line ≤ 1<br>	∧ -1.reserved = {<br>	∧ +1.roles not in {MAP}<br>	∧ +1.length ≥ 2<br>	∧ +3.reserved not in {}}<br>	∧ +4.reserved = (<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ⏎␣⁺␣⁺␣⁺␣⁺<br>Confidence: 0.940. Support: 323.` |
| 32 | `  •••start_col ≤ 81<br>	∧ •••start_line ≤ 240<br>	∧ -1.label in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ +1.length ≥ 2<br>	∧ +4.roles not in {EXPRESSION}<br>	∧ ^1.roles not in {IDENTIFIER}<br>	∧ ^2.roles in {CALL, EXPRESSION}<br>⇒ y = "<br>Confidence: 0.984. Support: 96.` |
| 33 | `  -1.label in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -4.diff_col ≤ 10<br>	∧ -5.roles not in {CALLEE}<br>	∧ -5.length ≥ 7<br>	∧ +1.length ≥ 2<br>	∧ +5.length ≤ 34<br>	∧ ^1.roles not in {IDENTIFIER}<br>	∧ ^2.roles not in {EXPRESSION}<br>⇒ y = "<br>Confidence: 0.996. Support: 2747.` |
| 34 | `  -1.label in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -4.diff_col ≤ 10<br>	∧ -5.roles not in {CALLEE}<br>	∧ -5.length ≤ 6<br>	∧ +1.length ≥ 10<br>	∧ +4.roles not in {EXPRESSION}<br>	∧ +5.length ≤ 34<br>	∧ ^1.roles not in {IDENTIFIER}<br>	∧ ^2.roles not in {EXPRESSION}<br>⇒ y = "<br>Confidence: 0.951. Support: 154.` |
| 35 | `  -1.label in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -4.diff_col ≤ 10<br>	∧ -5.roles not in {CALLEE}<br>	∧ -5.length ≤ 6<br>	∧ +1.roles in {RIGHT}<br>	∧ +1.length ≤ 9<br>	∧ +4.roles not in {EXPRESSION}<br>	∧ +5.length ≤ 34<br>	∧ ^1.roles not in {IDENTIFIER}<br>	∧ ^2.roles not in {EXPRESSION}<br>⇒ y = "<br>Confidence: 0.926. Support: 102.` |
| 36 | `  -1.label in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -4.diff_col ≤ 10<br>	∧ -5.roles not in {CALLEE}<br>	∧ -5.length ≤ 6<br>	∧ +1.length ≥ 2<br>	∧ +4.roles not in {EXPRESSION}<br>	∧ +5.length ≤ 4<br>	∧ ^1.roles not in {IDENTIFIER}<br>	∧ ^2.roles not in {EXPRESSION}<br>⇒ y = "<br>Confidence: 0.953. Support: 1679.` |
| 37 | `  -1.internal_type = CommentLine<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -3.label not in {<space>}<br>	∧ +1.reserved not in {function}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type = File<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ⏎<br>Confidence: 0.971. Support: 1979.` |
| 38 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.length ≥ 3<br>	∧ +1.roles in {EXPRESSION}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type = File<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 698.` |
| 39 | `  -1.diff_offset ≥ 2<br>	∧ -1.internal_type not in {CommentLine}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -2.roles in {COMMENT}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type = File<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ⏎<br>Confidence: 0.953. Support: 224.` |
| 40 | `  -1.diff_offset ≥ 2<br>	∧ -1.internal_type not in {CommentLine}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type = File<br>⇒ y = ∅<br>Confidence: 0.981. Support: 3317.` |
| 41 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.length ≤ 5<br>	∧ +1.roles in {KEY}<br>	∧ +1.length ≥ 2<br>	∧ +4.reserved not in {}}<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles not in {IDENTIFIER}<br>	∧ ^2.roles not in {CALL}<br>⇒ y = ⏎<br>Confidence: 0.977. Support: 2247.` |
| 42 | `  -1.diff_col ≥ 3<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ +1.roles in {EXPRESSION} and not in {KEY}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type = BlockStatement<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.991. Support: 7849.` |
| 43 | `  •••start_col ≥ 52<br>	∧ -1.diff_col ≤ 2<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -5.label not in {<newline>}<br>	∧ +1.roles not in {KEY}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type = BlockStatement<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ⏎<br>Confidence: 0.973. Support: 1419.` |
| 44 | `  •••start_col ≤ 51<br>	∧ -1.diff_col ≤ 2<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -5.label not in {<newline>}<br>	∧ +1.roles not in {KEY}<br>	∧ +1.length ≥ 2<br>	∧ +2.reserved = .<br>	∧ ^1.internal_type = BlockStatement<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ⏎<br>Confidence: 0.933. Support: 319.` |
| 45 | `  •••start_col ≤ 25<br>	∧ -1.diff_col ≤ 2<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ +1.reserved = function<br>	∧ +1.roles not in {KEY}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type = BlockStatement<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ⏎⏎<br>Confidence: 0.981. Support: 1077.` |
| 46 | `  •••start_col ≤ 25<br>	∧ -1.diff_col ≤ 2<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -2.label in {<-space>}<br>	∧ +1.reserved not in {function}<br>	∧ +1.roles not in {KEY}<br>	∧ +1.length ≥ 2<br>	∧ +3.length ≤ 14<br>	∧ +5.reserved = function<br>	∧ ^1.internal_type = BlockStatement<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ⏎<br>Confidence: 0.977. Support: 153.` |
| 47 | `  •••start_col ≤ 25<br>	∧ •••start_line ≤ 23<br>	∧ -1.diff_col ≤ 2<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -2.label in {<-space>}<br>	∧ +1.reserved not in {function}<br>	∧ +1.roles not in {KEY}<br>	∧ +1.length ≥ 2<br>	∧ +2.length ≤ 9<br>	∧ +3.length ≤ 14<br>	∧ +5.reserved not in {function}<br>	∧ ^1.internal_type = BlockStatement<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ⏎<br>Confidence: 0.995. Support: 104.` |
| 48 | `  •••start_col ≤ 25<br>	∧ -1.diff_col ≤ 2<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ), ;, {}<br>	∧ -2.label not in {<-space>}<br>	∧ +1.reserved not in {function}<br>	∧ +1.roles not in {KEY}<br>	∧ +1.length ≥ 2<br>	∧ +3.length ≤ 14<br>	∧ ^1.internal_type = BlockStatement<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.987. Support: 276.` |
| 49 | `  -1.label in {<newline>} and not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -2.diff_offset ≥ 20<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {BlockStatement, File}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = "<br>Confidence: 0.998. Support: 251.` |
| 50 | `  •••start_line ≥ 218<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = [<br>	∧ -3.diff_offset ≥ 4<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {BlockStatement, File}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = "<br>Confidence: 0.925. Support: 154.` |
| 51 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<newline>, <space>}<br>	∧ -1.reserved = +<br>	∧ -2.label in {<space>}<br>	∧ -3.diff_offset ≥ 4<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {BlockStatement, File}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.992. Support: 311.` |
| 52 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<newline>, <space>}<br>	∧ -1.reserved = +<br>	∧ -2.label not in {<space>}<br>	∧ -3.diff_offset ≥ 4<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {BlockStatement, File}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ⏎<br>Confidence: 0.935. Support: 208.` |
| 53 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<newline>, <space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -3.diff_offset ≥ 4<br>	∧ -3.length ≥ 2<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {BlockStatement, File}<br>	∧ ^1.roles not in {IDENTIFIER}<br>	∧ ^2.roles in {ARGUMENT}<br>⇒ y = ␣<br>Confidence: 0.977. Support: 320.` |
| 54 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<newline>, <space>}<br>	∧ -1.reserved = )<br>	∧ -3.diff_offset ≥ 4<br>	∧ +1.length ≤ 4<br>	∧ ^1.internal_type not in {BlockStatement, File}<br>	∧ ^1.roles not in {IDENTIFIER}<br>	∧ ^2.roles not in {ARGUMENT}<br>⇒ y = ␣<br>Confidence: 0.994. Support: 624.` |
| 55 | `  •••start_line ≥ 63<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<newline>, <space>}<br>	∧ -1.reserved = :<br>	∧ -3.diff_offset ≥ 4<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {BlockStatement, File}<br>	∧ ^1.roles not in {IDENTIFIER}<br>	∧ ^2.roles not in {ARGUMENT, STATEMENT}<br>⇒ y = ␣<br>Confidence: 0.961. Support: 2869.` |
| 56 | `  •••start_line ≤ 63<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<newline>, <space>}<br>	∧ -1.reserved = :<br>	∧ -2.length ≤ 4<br>	∧ -3.diff_offset ≥ 4<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {BlockStatement, File}<br>	∧ ^1.roles not in {IDENTIFIER}<br>	∧ ^2.roles not in {ARGUMENT, STATEMENT}<br>⇒ y = ␣<br>Confidence: 0.977. Support: 151.` |
| 57 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<newline>, <space>}<br>	∧ -1.reserved not in {(, ), ;, {}<br>	∧ -2.diff_col ≥ 3<br>	∧ -3.diff_offset ≥ 4<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type = LogicalExpression<br>	∧ ^1.roles not in {IDENTIFIER}<br>	∧ ^2.roles not in {ARGUMENT}<br>⇒ y = ␣<br>Confidence: 0.947. Support: 2701.` |
| 58 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<newline>, <space>}<br>	∧ -1.reserved not in {(, ), ;, {}<br>	∧ -3.diff_offset ≥ 4<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {BlockStatement, File}<br>	∧ ^1.roles not in {IDENTIFIER}<br>	∧ ^2.roles not in {ARGUMENT}<br>⇒ y = ␣<br>Confidence: 0.993. Support: 43869.` |
| 59 | `  -1.label not in {<newline>, <space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -3.diff_offset ≤ 3<br>	∧ -5.diff_col ≤ 34<br>	∧ +1.roles not in {KEY}<br>	∧ +1.length ≥ 2<br>	∧ +3.reserved = {<br>	∧ ^1.internal_type not in {BlockStatement, File}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.936. Support: 351.` |
| 60 | `  •••start_col ≤ 122<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -2.reserved not in {(}<br>	∧ -3.diff_offset ≤ 3<br>	∧ -4.roles in {RIGHT}<br>	∧ -5.diff_col ≤ 34<br>	∧ +1.roles in {STRING}<br>	∧ +1.length ≥ 2<br>	∧ +3.reserved not in {{}<br>	∧ ^1.internal_type not in {BlockStatement, File}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ⏎<br>Confidence: 0.932. Support: 155.` |
| 61 | `  -1.label not in {<newline>, <space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -2.reserved not in {(}<br>	∧ -3.diff_offset ≤ 3<br>	∧ -5.diff_col ≤ 34<br>	∧ +1.length ≥ 2<br>	∧ +3.reserved not in {{}<br>	∧ ^1.internal_type not in {BlockStatement, File}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.946. Support: 3628.` |
| 62 | `  -1.label not in {<newline>, <space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -3.diff_offset ≤ 3<br>	∧ -5.diff_col ≤ 4<br>	∧ +1.roles not in {KEY}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {BlockStatement, File}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.923. Support: 615.` |
| 63 | `  -1.diff_offset ≥ 22<br>	∧ -1.internal_type = StringLiteral<br>	∧ -2.label in {<space>}<br>	∧ +1.length ≤ 1<br>	∧ +4.internal_type = Identifier<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = "<br>Confidence: 0.963. Support: 452.` |
| 64 | `  -1.diff_offset ≥ 22<br>	∧ -1.internal_type = StringLiteral<br>	∧ -2.label not in {<space>}<br>	∧ -3.length ≥ 5<br>	∧ +1.length ≤ 1<br>	∧ +3.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = "<br>Confidence: 0.937. Support: 279.` |
| 65 | `  -1.diff_offset ≥ 22<br>	∧ -1.internal_type = StringLiteral<br>	∧ -2.label not in {<space>}<br>	∧ -2.length ≥ 19<br>	∧ -3.length ≤ 4<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER}<br>	∧ ^2.internal_type not in {TryStatement}<br>⇒ y = "<br>Confidence: 0.996. Support: 124.` |
| 66 | `  -1.diff_offset ≥ 22<br>	∧ -1.internal_type = StringLiteral<br>	∧ -2.label not in {<space>}<br>	∧ -2.length ≤ 19<br>	∧ -3.length ≤ 4<br>	∧ -4.label in {<space>}<br>	∧ +1.length ≤ 1<br>	∧ +4.reserved not in {.}<br>	∧ ^1.roles not in {IDENTIFIER}<br>	∧ ^2.internal_type not in {TryStatement}<br>⇒ y = '<br>Confidence: 0.939. Support: 2115.` |
| 67 | `  •••start_col ≥ 37<br>	∧ -1.diff_offset ≤ 21<br>	∧ -1.internal_type = StringLiteral<br>	∧ -1.length ≥ 7<br>	∧ -4.length ≤ 6<br>	∧ +1.length ≤ 1<br>	∧ +2.length ≥ 4<br>	∧ +5.length ≤ 4<br>	∧ ^1.roles not in {IDENTIFIER, IF}<br>	∧ ^2.roles in {CALL}<br>⇒ y = "<br>Confidence: 0.958. Support: 297.` |
| 68 | `  •••start_col ≥ 37<br>	∧ -1.diff_offset ≤ 21<br>	∧ -1.internal_type = StringLiteral<br>	∧ -4.length ≤ 6<br>	∧ +1.length ≤ 1<br>	∧ +2.length ≥ 4<br>	∧ +5.length ≤ 4<br>	∧ ^1.roles not in {IDENTIFIER, IF}<br>	∧ ^2.roles not in {CALL}<br>⇒ y = "<br>Confidence: 0.985. Support: 6449.` |
| 69 | `  •••start_col ≥ 37<br>	∧ -1.diff_offset ≤ 21<br>	∧ -1.internal_type = StringLiteral<br>	∧ +1.length ≤ 1<br>	∧ +2.reserved = (<br>	∧ +2.length ≤ 3<br>	∧ +3.internal_type = Identifier<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = '<br>Confidence: 0.924. Support: 192.` |
| 70 | `  •••start_col ≥ 37<br>	∧ -1.diff_offset ≤ 21<br>	∧ -1.internal_type = StringLiteral<br>	∧ +1.length ≤ 1<br>	∧ +2.reserved = (<br>	∧ +2.length ≤ 3<br>	∧ +3.internal_type not in {Identifier}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = "<br>Confidence: 0.933. Support: 97.` |
| 71 | `  •••start_col ≥ 37<br>	∧ -1.diff_offset ≤ 21<br>	∧ -1.internal_type = StringLiteral<br>	∧ -3.reserved not in {)}<br>	∧ -3.length ≤ 12<br>	∧ -4.internal_type = Identifier<br>	∧ +1.reserved = }<br>	∧ +1.length ≤ 1<br>	∧ +2.reserved not in {(}<br>	∧ +2.length ≤ 3<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = '<br>Confidence: 0.995. Support: 107.` |
| 72 | `  •••start_col ≥ 37<br>	∧ -1.diff_offset ≤ 21<br>	∧ -1.internal_type = StringLiteral<br>	∧ -3.reserved not in {)}<br>	∧ -3.length ≤ 12<br>	∧ -4.internal_type = Identifier<br>	∧ -5.diff_offset ≤ 12<br>	∧ +1.reserved not in {}}<br>	∧ +1.length ≤ 1<br>	∧ +2.reserved not in {(}<br>	∧ +2.length ≤ 3<br>	∧ +3.reserved not in {+}<br>	∧ ^1.roles in {MAP} and not in {IDENTIFIER}<br>⇒ y = "<br>Confidence: 0.994. Support: 90.` |
| 73 | `  •••start_col ≥ 46<br>	∧ -1.diff_offset ≤ 21<br>	∧ -1.internal_type = StringLiteral<br>	∧ -3.reserved not in {)}<br>	∧ -3.length ≤ 12<br>	∧ -4.internal_type not in {Identifier}<br>	∧ -5.reserved = =<br>	∧ +1.length ≤ 1<br>	∧ +2.reserved not in {(}<br>	∧ +2.length ≤ 3<br>	∧ +3.reserved not in {+}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = "<br>Confidence: 0.995. Support: 96.` |
| 74 | `  •••start_col ≤ 62<br>	∧ •••start_line ≥ 115<br>	∧ -1.diff_offset ≤ 21<br>	∧ -1.internal_type = StringLiteral<br>	∧ -3.reserved not in {)}<br>	∧ -3.length ≤ 12<br>	∧ -4.internal_type not in {Identifier}<br>	∧ -5.reserved not in {=}<br>	∧ +1.reserved = ]<br>	∧ +1.length ≤ 1<br>	∧ +2.reserved not in {(}<br>	∧ +2.length ≤ 3<br>	∧ +3.reserved not in {+}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = "<br>Confidence: 0.932. Support: 139.` |
| 75 | `  •••start_col ≤ 62<br>	∧ -1.diff_offset ≤ 21<br>	∧ -1.internal_type = StringLiteral<br>	∧ -3.reserved not in {)}<br>	∧ -3.length ≤ 12<br>	∧ -4.internal_type not in {Identifier}<br>	∧ -5.reserved not in {=}<br>	∧ +1.reserved not in {]}<br>	∧ +1.length ≤ 1<br>	∧ +2.reserved not in {(}<br>	∧ +2.length ≤ 3<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = "<br>Confidence: 0.927. Support: 6766.` |
| 76 | `  •••start_col ≤ 38<br>	∧ -1.diff_offset ≤ 21<br>	∧ -1.internal_type = StringLiteral<br>	∧ -3.diff_col ≥ 9<br>	∧ -3.reserved not in {)}<br>	∧ -3.length ≤ 12<br>	∧ -4.internal_type not in {Identifier}<br>	∧ -5.diff_col ≥ 18<br>	∧ +1.length ≤ 1<br>	∧ +2.reserved not in {(}<br>	∧ +2.length ≤ 3<br>	∧ +3.reserved not in {+}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = "<br>Confidence: 0.965. Support: 156.` |
| 77 | `  •••start_col ≤ 38<br>	∧ -1.diff_offset ≤ 21<br>	∧ -1.internal_type = StringLiteral<br>	∧ -3.diff_col ≥ 9<br>	∧ -3.reserved not in {)}<br>	∧ -3.length ≤ 12<br>	∧ -4.internal_type not in {Identifier}<br>	∧ -5.diff_col ≤ 17<br>	∧ +1.length ≤ 1<br>	∧ +2.reserved not in {(}<br>	∧ +2.length ≤ 3<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = '<br>Confidence: 0.956. Support: 195.` |
| 78 | `  •••start_col ≤ 38<br>	∧ -1.diff_offset ≤ 21<br>	∧ -1.internal_type = StringLiteral<br>	∧ -3.diff_col ≤ 9<br>	∧ -3.reserved not in {)}<br>	∧ -3.length ≤ 12<br>	∧ -4.internal_type not in {Identifier}<br>	∧ +1.length ≤ 1<br>	∧ +2.reserved not in {(}<br>	∧ +2.length ≤ 3<br>	∧ +3.reserved not in {+}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = "<br>Confidence: 0.998. Support: 220.` |
| 79 | `  •••start_col ≤ 36<br>	∧ •••start_line ≤ 109<br>	∧ -1.diff_offset ≤ 21<br>	∧ -1.internal_type = StringLiteral<br>	∧ -5.diff_offset ≥ 30<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = '<br>Confidence: 0.953. Support: 715.` |
| 80 | `  •••start_col ≤ 36<br>	∧ •••start_line ≤ 142<br>	∧ -1.diff_offset ≤ 21<br>	∧ -1.internal_type = StringLiteral<br>	∧ -1.roles in {ARGUMENT}<br>	∧ -5.diff_offset ≤ 29<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = "<br>Confidence: 0.937. Support: 1005.` |
| 81 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {MAP}<br>	∧ -5.diff_offset ≤ 18<br>	∧ +1.reserved = }<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.973. Support: 201.` |
| 82 | `  •••start_col ≥ 9<br>	∧ -1.diff_col ≤ 3<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.roles not in {MAP}<br>	∧ -3.internal_type = StringLiteral<br>	∧ -3.roles in {CALL}<br>	∧ -5.length ≤ 35<br>	∧ +1.reserved = }<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {BINARY, IDENTIFIER}<br>⇒ y = ⏎␣⁻␣⁻␣⁻␣⁻<br>Confidence: 0.984. Support: 411.` |
| 83 | `  •••start_col ≥ 9<br>	∧ -1.diff_col ≤ 3<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ;<br>	∧ -1.roles not in {MAP}<br>	∧ -3.internal_type = StringLiteral<br>	∧ -3.roles not in {CALL}<br>	∧ -5.length ≤ 35<br>	∧ +1.reserved = }<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {BINARY, IDENTIFIER}<br>⇒ y = ⏎␣⁻␣⁻␣⁻␣⁻<br>Confidence: 0.937. Support: 342.` |
| 84 | `  •••start_col ≥ 9<br>	∧ -1.diff_col ≤ 3<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.roles not in {MAP}<br>	∧ -3.internal_type not in {StringLiteral}<br>	∧ -5.length ≤ 35<br>	∧ +1.reserved = }<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {BINARY, IDENTIFIER}<br>⇒ y = ⏎␣⁻␣⁻␣⁻␣⁻<br>Confidence: 0.974. Support: 13954.` |
| 85 | `  •••start_col ≤ 9<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.roles not in {MAP}<br>	∧ -2.label in {<-space>}<br>	∧ +1.reserved = }<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ⏎␣⁻␣⁻␣⁻␣⁻<br>Confidence: 0.995. Support: 94.` |
| 86 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = =<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.994. Support: 13873.` |
| 87 | `  •••start_line ≥ 11<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -3.reserved = function<br>	∧ -5.reserved = ,<br>	∧ +1.reserved = {<br>	∧ +1.length ≤ 1<br>	∧ +5.internal_type = NumericLiteral<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ⏎<br>Confidence: 0.998. Support: 237.` |
| 88 | `  •••start_line ≥ 11<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -3.reserved = function<br>	∧ -4.reserved = (<br>	∧ -5.reserved not in {,}<br>	∧ +1.reserved = {<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.964. Support: 125.` |
| 89 | `  •••start_line ≤ 10<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -3.reserved = function<br>	∧ +1.reserved = {<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣␣<br>Confidence: 0.929. Support: 163.` |
| 90 | `  •••start_line ≤ 4<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -3.reserved = function<br>	∧ +1.reserved = {<br>	∧ +1.length ≤ 1<br>	∧ +2.length ≤ 6<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.960. Support: 210.` |
| 91 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = [<br>	∧ -3.reserved not in {function}<br>	∧ +1.reserved = {<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 168.` |
| 92 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, [}<br>	∧ -4.length ≤ 4<br>	∧ +1.reserved = {<br>	∧ +1.length ≤ 1<br>	∧ +2.length ≥ 2<br>	∧ ^1.roles in {ARGUMENT} and not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.993. Support: 1719.` |
| 93 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, [}<br>	∧ +1.reserved = {<br>	∧ +1.length ≤ 1<br>	∧ +2.length ≥ 2<br>	∧ ^1.roles not in {ARGUMENT, IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.984. Support: 12247.` |
| 94 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -2.label not in {<space>}<br>	∧ +1.reserved = )<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type = BinaryExpression<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 450.` |
| 95 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.roles in {CALLEE}<br>	∧ +1.reserved not in {), =, {, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type = BinaryExpression<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.995. Support: 91.` |
| 96 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.roles not in {CALLEE}<br>	∧ -2.reserved not in {[}<br>	∧ +1.reserved not in {), }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type = BinaryExpression<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.973. Support: 5357.` |
| 97 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = if<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {BinaryExpression}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.964. Support: 4355.` |
| 98 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles in {EXPRESSION} and not in {STRING}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {BinaryExpression}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 911.` |
| 99 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = ,<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles in {EXPRESSION, KEY}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {BinaryExpression}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ⏎<br>Confidence: 0.997. Support: 186.` |
| 100 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, if}<br>	∧ -2.reserved = )<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type = FunctionExpression<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ⏎␣⁺␣⁺␣⁺␣⁺<br>Confidence: 0.996. Support: 136.` |
| 101 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, if}<br>	∧ -2.reserved = )<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {BinaryExpression, FunctionExpression}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ⏎<br>Confidence: 0.974. Support: 97.` |
| 102 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, if}<br>	∧ -2.reserved not in {)}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {BinaryExpression}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.946. Support: 3171.` |
| 103 | `  •••start_line ≥ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = function<br>	∧ -5.roles in {MAP}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {BinaryExpression}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.945. Support: 173.` |
| 104 | `  •••start_col ≤ 78<br>	∧ •••start_line ≤ 177<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = function<br>	∧ -4.reserved = '<br>	∧ -5.roles not in {MAP}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ +4.roles not in {EXPRESSION}<br>	∧ +5.reserved not in {var}<br>	∧ +5.length ≤ 2<br>	∧ ^1.internal_type not in {BinaryExpression}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.996. Support: 123.` |
| 105 | `  •••start_line ≥ 80<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = function<br>	∧ -4.reserved not in {'}<br>	∧ -5.length ≥ 16<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {BinaryExpression}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.944. Support: 940.` |
| 106 | `  •••start_col ≤ 57<br>	∧ •••start_line ≥ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = function<br>	∧ -4.reserved not in {'}<br>	∧ -5.diff_line ≤ 1<br>	∧ -5.roles not in {MAP}<br>	∧ -5.length ≤ 15<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ +5.internal_type = Identifier<br>	∧ +5.length ≤ 2<br>	∧ ^1.internal_type not in {AssignmentExpression, BinaryExpression}<br>	∧ ^1.roles not in {IDENTIFIER}<br>	∧ ^2.roles not in {CALL}<br>⇒ y = ∅<br>Confidence: 0.996. Support: 114.` |
| 107 | `  •••start_line ≤ 1<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = function<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {BinaryExpression}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.940. Support: 410.` |
| 108 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = :<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {BinaryExpression}<br>	∧ ^1.roles not in {IDENTIFIER}<br>	∧ ^2.roles not in {ARGUMENT}<br>⇒ y = ␣<br>Confidence: 0.936. Support: 303.` |
| 109 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {if}<br>	∧ -2.internal_type = StringLiteral<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {BinaryExpression}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.943. Support: 184.` |
| 110 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {BinaryExpression}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.991. Support: 158.` |
| 111 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = =<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {BinaryExpression}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.990. Support: 143.` |
| 112 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = return<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {BinaryExpression}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.931. Support: 137.` |
| 113 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, =, function, if, return}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type = CallExpression<br>	∧ ^1.roles in {CONDITION} and not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 926.` |
| 114 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {return}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ +2.roles in {EXPRESSION}<br>	∧ ^1.internal_type not in {BinaryExpression, CallExpression}<br>	∧ ^1.roles in {CONDITION} and not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.940. Support: 440.` |
| 115 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {return}<br>	∧ -2.length ≥ 9<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ +5.internal_type = NumericLiteral<br>	∧ ^1.internal_type not in {BinaryExpression}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.942. Support: 269.` |
| 116 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, =, function, if, return}<br>	∧ -2.length ≤ 8<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ +5.internal_type = NumericLiteral<br>	∧ ^1.internal_type not in {BinaryExpression}<br>	∧ ^1.roles not in {CONDITION, IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.977. Support: 490.` |
| 117 | `  •••start_line ≥ 34<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, =, function, if, return}<br>	∧ -1.roles in {IDENTIFIER}<br>	∧ -1.length ≥ 5<br>	∧ -2.length ≥ 13<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {BinaryExpression}<br>	∧ ^1.roles not in {CONDITION, IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.979. Support: 2935.` |
| 118 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, =, function, if, return}<br>	∧ -1.length ≤ 4<br>	∧ -2.length ≥ 13<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {BinaryExpression}<br>	∧ ^1.roles not in {CONDITION, IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.987. Support: 4627.` |
| 119 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, =, function, if, return}<br>	∧ -2.label not in {<space>}<br>	∧ -2.length ≤ 12<br>	∧ -3.reserved = }<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {BinaryExpression}<br>	∧ ^1.roles not in {CONDITION, IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.961. Support: 748.` |
| 120 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, =, function, if, return}<br>	∧ -2.length ≤ 12<br>	∧ -3.reserved = function<br>	∧ +1.reserved not in {=, {}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {BinaryExpression}<br>	∧ ^1.roles not in {CONDITION, IDENTIFIER}<br>	∧ ^2.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.972. Support: 4895.` |
| 121 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, =, function, if, return}<br>	∧ -2.label not in {<space>}<br>	∧ -2.length ≤ 12<br>	∧ -3.reserved not in {}}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type = LogicalExpression<br>	∧ ^1.roles not in {CONDITION, IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 188.` |
| 122 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, =, function, if, return}<br>	∧ -2.length ≤ 12<br>	∧ -3.reserved not in {}}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {BinaryExpression, LogicalExpression}<br>	∧ ^1.roles not in {CONDITION, IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 55282.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 10.336065573770492, "max_conf": 0.9997459650039673, "max_support": 55282, "min_conf": 0.9207746386528015, "min_support": 90, "num_rules": 122}}
```
</details>
