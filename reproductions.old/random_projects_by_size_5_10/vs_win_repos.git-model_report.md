# Model report for file:///tmp/top-repos-quality-repos-3r43hcge/vs_win_repos.git HEAD 84dbbacbbb5a990a0b04097ebc70ddbc883e56b9

### Dump

```json
{'created_at': '2021-08-21 17:22:08',
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
 'size': '21.2 kB',
 'tags': [],
 'uuid': '2be4402b-8fe9-440e-87b6-7cfe6dd48777',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-3r43hcge/vs_win_repos.git 84dbbacbbb5a990a0b04097ebc70ddbc883e56b9

# javascript
61 rules, avg.len. 12.2
## train
PPCR: 0.936267
### report
macro
{'f1-score': 0.6295769795580259,
 'precision': 0.6881226494928282,
 'recall': 0.5989723312463229,
 'support': 187450}
micro
{'f1-score': 0.9671912510002667,
 'precision': 0.9671912510002667,
 'recall': 0.9671912510002667,
 'support': 187450}
weighted
{'f1-score': 0.9652055460519825,
 'precision': 0.965647336963604,
 'recall': 0.9671912510002667,
 'support': 187450}
### report_full
macro
{'f1-score': 0.5444246552794318,
 'precision': 0.6881226494928282,
 'recall': 0.5021127123244555,
 'support': 200210}
micro
{'f1-score': 0.9353557240881185,
 'precision': 0.9671912510002667,
 'recall': 0.9055491733679636,
 'support': 200210}
weighted
{'f1-score': 0.9223449414095428,
 'precision': 0.9594827397258319,
 'recall': 0.9055491733679636,
 'support': 200210}
## test
PPCR: 0.936267
### report
macro
{'f1-score': 0.6295769795580259,
 'precision': 0.6881226494928282,
 'recall': 0.5989723312463229,
 'support': 37490}
micro
{'f1-score': 0.9671912510002667,
 'precision': 0.9671912510002667,
 'recall': 0.9671912510002667,
 'support': 37490}
weighted
{'f1-score': 0.9652055460519826,
 'precision': 0.9656473369636043,
 'recall': 0.9671912510002667,
 'support': 37490}
### report_full
macro
{'f1-score': 0.5444246552794318,
 'precision': 0.6881226494928282,
 'recall': 0.5021127123244555,
 'support': 40042}
micro
{'f1-score': 0.9353557240881185,
 'precision': 0.9671912510002667,
 'recall': 0.9055491733679636,
 'support': 40042}
weighted
{'f1-score': 0.9223449414095427,
 'precision': 0.9594827397258321,
 'recall': 0.9055491733679636,
 'support': 40042}
```

## javascript
### Summary
43 rules, avg.len. 11.6

| | |
|-|-|
|Min support|102|
|Max support|37020|
|Min confidence|0.9247787594795227|
|Max confidence|0.9999452233314514|

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
| 1 | `  -1.reserved = [<br>	∧ +1.length ≥ 3<br>	∧ ^1.roles in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.960. Support: 969.` |
| 2 | `  •••start_col ≤ 21<br>	∧ -1.reserved = [<br>	∧ -2.roles in {IDENTIFIER}<br>	∧ -2.length ≤ 7<br>	∧ +1.length ≤ 2<br>	∧ +3.reserved not in {,, ., =}<br>	∧ ^1.roles in {IDENTIFIER}<br>	∧ ^2.roles not in {CONDITION}<br>⇒ y = ␣<br>Confidence: 0.949. Support: 108.` |
| 3 | `  -1.reserved = [<br>	∧ -2.roles not in {IDENTIFIER}<br>	∧ +1.length ≤ 2<br>	∧ ^1.roles in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.970. Support: 152.` |
| 4 | `  -1.internal_type = StringLiteral<br>	∧ -1.reserved not in {[}<br>	∧ +1.reserved = )<br>	∧ ^1.roles in {IDENTIFIER}<br>⇒ y = "<br>Confidence: 0.996. Support: 127.` |
| 5 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {[}<br>	∧ -2.length ≤ 4<br>	∧ +1.reserved = )<br>	∧ ^1.roles in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.927. Support: 415.` |
| 6 | `  -1.reserved not in {[}<br>	∧ -3.length ≤ 2<br>	∧ +1.reserved = ]<br>	∧ ^1.roles in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.957. Support: 175.` |
| 7 | `  -1.reserved not in {[}<br>	∧ +1.reserved not in {), ]}<br>	∧ ^1.roles in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.995. Support: 20951.` |
| 8 | `  -1.roles in {STRING}<br>	∧ +2.roles in {ARGUMENT}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = "<br>Confidence: 0.997. Support: 174.` |
| 9 | `  -1.roles not in {STRING}<br>	∧ +2.roles in {ARGUMENT}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 1.000. Support: 9129.` |
| 10 | `  -1.roles in {STRING}<br>	∧ +1.reserved = ;<br>	∧ +2.roles not in {ARGUMENT}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = "<br>Confidence: 0.998. Support: 254.` |
| 11 | `  -1.reserved = (<br>	∧ -1.roles not in {STRING}<br>	∧ +1.reserved = ;<br>	∧ +2.roles not in {ARGUMENT}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 150.` |
| 12 | `  -1.reserved not in {(}<br>	∧ -1.roles not in {STRING}<br>	∧ +1.reserved = ;<br>	∧ +2.roles not in {ARGUMENT}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 7694.` |
| 13 | `  -1.internal_type = CommentLine<br>	∧ +1.reserved not in {;}<br>	∧ +2.roles not in {ARGUMENT}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ⏎<br>Confidence: 0.992. Support: 5261.` |
| 14 | `  -1.internal_type not in {CommentLine}<br>	∧ +1.reserved not in {;, }}<br>	∧ +1.roles in {COMMENT}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 1.000. Support: 5396.` |
| 15 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.reserved = {<br>	∧ +1.reserved not in {;, }}<br>	∧ +1.roles not in {COMMENT}<br>	∧ +2.roles not in {ARGUMENT, COMMENT}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ⏎⇥⁺<br>Confidence: 0.937. Support: 4878.` |
| 16 | `  -1.internal_type = StringLiteral<br>	∧ -1.reserved not in {{}<br>	∧ +1.reserved = ,<br>	∧ +1.roles not in {COMMENT}<br>	∧ +2.roles not in {ARGUMENT}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = "<br>Confidence: 0.999. Support: 351.` |
| 17 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {{}<br>	∧ +1.reserved = ,<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 4019.` |
| 18 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.label in {<space>}<br>	∧ -1.reserved not in {{}<br>	∧ +1.reserved not in {,, ;, }}<br>	∧ +1.roles not in {COMMENT}<br>	∧ +2.roles not in {ARGUMENT}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = "<br>Confidence: 1.000. Support: 2744.` |
| 19 | `  -1.internal_type = StringLiteral<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {{}<br>	∧ +1.reserved not in {,, ;, }}<br>	∧ +1.roles not in {COMMENT}<br>	∧ +2.roles not in {ARGUMENT}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = "<br>Confidence: 1.000. Support: 2411.` |
| 20 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {;, {}<br>	∧ -1.roles in {MAP}<br>	∧ +1.reserved not in {;, }}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 1.000. Support: 1444.` |
| 21 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = !<br>	∧ -1.roles not in {MAP}<br>	∧ +1.reserved not in {}}<br>	∧ +2.roles not in {COMMENT}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 993.` |
| 22 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {;, {}<br>	∧ -1.length ≥ 2<br>	∧ +1.reserved not in {,, ;, }}<br>	∧ +1.roles in {KEY} and not in {COMMENT}<br>	∧ +2.roles not in {ARGUMENT, COMMENT}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = "<br>Confidence: 0.998. Support: 269.` |
| 23 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {!, ;, {}<br>	∧ -1.roles in {IDENTIFIER} and not in {MAP}<br>	∧ +1.reserved = (<br>	∧ +2.roles not in {COMMENT}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 1.000. Support: 2086.` |
| 24 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {!, ;, {}<br>	∧ -1.roles not in {IDENTIFIER, MAP}<br>	∧ +1.reserved = (<br>	∧ +2.reserved = )<br>	∧ +2.roles not in {COMMENT}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 418.` |
| 25 | `  •••start_col ≤ 34<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {!, {}<br>	∧ -1.roles not in {IDENTIFIER, MAP}<br>	∧ +1.reserved = (<br>	∧ +1.roles not in {COMMENT, KEY}<br>	∧ +2.reserved not in {)}<br>	∧ +2.roles not in {ARGUMENT, COMMENT}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.976. Support: 3822.` |
| 26 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {!, ;, {}<br>	∧ -1.roles not in {MAP}<br>	∧ -2.label in {<-tab>}<br>	∧ +1.reserved not in {(, ), ,, ;, }}<br>	∧ +1.roles not in {COMMENT, KEY}<br>	∧ +2.roles not in {ARGUMENT, COMMENT}<br>	∧ ^1.roles not in {BLOCK, IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.992. Support: 438.` |
| 27 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {!, ;, {}<br>	∧ -1.roles not in {MAP}<br>	∧ -2.label not in {<-tab>}<br>	∧ -2.reserved = [<br>	∧ +1.reserved not in {(, }}<br>	∧ +2.roles not in {COMMENT}<br>	∧ ^1.internal_type not in {VariableDeclaration}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.950. Support: 369.` |
| 28 | `  -1.diff_offset ≥ 42<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {!, ;, {}<br>	∧ -1.roles not in {MAP}<br>	∧ -2.diff_line ≥ 1<br>	∧ -2.label not in {<-tab>}<br>	∧ +1.reserved not in {(, }}<br>	∧ +2.roles not in {COMMENT}<br>	∧ ^1.internal_type not in {VariableDeclaration}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.995. Support: 102.` |
| 29 | `  -1.diff_offset ≤ 42<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;, {}<br>	∧ -2.diff_line ≥ 1<br>	∧ -2.label not in {<-tab>}<br>	∧ +1.reserved not in {(, ,, ;, }}<br>	∧ +1.roles not in {COMMENT}<br>	∧ +2.roles not in {ARGUMENT, COMMENT}<br>	∧ ^1.internal_type not in {VariableDeclaration}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = "<br>Confidence: 0.996. Support: 124.` |
| 30 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = (<br>	∧ -1.roles not in {MAP}<br>	∧ -2.label not in {<-tab>}<br>	∧ +1.reserved = )<br>	∧ +2.roles not in {COMMENT}<br>	∧ ^1.internal_type not in {VariableDeclaration}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 994.` |
| 31 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = (<br>	∧ -1.roles not in {MAP}<br>	∧ -2.diff_line = 0<br>	∧ -2.label not in {<-tab>}<br>	∧ +1.reserved = function<br>	∧ +1.roles not in {ASSIGNMENT, COMMENT}<br>	∧ +2.roles not in {ARGUMENT, COMMENT}<br>	∧ ^1.internal_type not in {VariableDeclaration}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.953. Support: 181.` |
| 32 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = (<br>	∧ -1.roles not in {MAP}<br>	∧ -2.diff_line = 0<br>	∧ -2.label not in {<-tab>}<br>	∧ +1.internal_type = StringLiteral<br>	∧ +1.reserved not in {(, ), ,, }}<br>	∧ +1.roles not in {ASSIGNMENT, COMMENT}<br>	∧ +2.reserved not in {)}<br>	∧ +2.roles not in {ARGUMENT, COMMENT}<br>	∧ ^1.internal_type not in {VariableDeclaration}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 276.` |
| 33 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = (<br>	∧ -1.roles not in {MAP}<br>	∧ -2.label not in {<-tab>}<br>	∧ +1.reserved not in {(, ), ,, }}<br>	∧ +1.roles not in {ASSIGNMENT, COMMENT}<br>	∧ +2.roles not in {ARGUMENT, COMMENT}<br>	∧ ^1.internal_type not in {VariableDeclaration}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.967. Support: 9241.` |
| 34 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = ?<br>	∧ -1.roles not in {MAP}<br>	∧ -2.diff_line = 0<br>	∧ -2.label not in {<-tab>}<br>	∧ +1.reserved not in {,, }}<br>	∧ +1.roles in {LITERAL} and not in {COMMENT}<br>	∧ +2.roles not in {ARGUMENT, COMMENT}<br>	∧ ^1.internal_type not in {VariableDeclaration}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.925. Support: 113.` |
| 35 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, ?, {}<br>	∧ -1.roles not in {MAP}<br>	∧ -2.diff_line = 0<br>	∧ -2.label not in {<-tab>}<br>	∧ -3.diff_offset ≥ 4<br>	∧ +1.reserved not in {(, ,, ;, }}<br>	∧ +1.roles in {EXPRESSION} and not in {COMMENT}<br>	∧ +2.roles not in {ARGUMENT, COMMENT}<br>	∧ +3.reserved = {<br>	∧ ^1.internal_type not in {VariableDeclaration}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 1701.` |
| 36 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, ?, {}<br>	∧ -1.roles not in {MAP}<br>	∧ -2.diff_line = 0<br>	∧ -2.label not in {<-tab>}<br>	∧ -3.diff_offset ≥ 4<br>	∧ -3.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {(, ,, ;, }}<br>	∧ +1.roles not in {COMMENT, EXPRESSION}<br>	∧ +2.roles not in {ARGUMENT, COMMENT}<br>	∧ +3.reserved = {<br>	∧ ^1.internal_type not in {VariableDeclaration}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.932. Support: 477.` |
| 37 | `  -1.internal_type = Identifier<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {!, (, ;, ?, {}<br>	∧ -2.label not in {<-tab>}<br>	∧ -3.diff_offset ≤ 3<br>	∧ +1.reserved not in {(, }}<br>	∧ +2.roles not in {COMMENT}<br>	∧ +3.reserved = {<br>	∧ ^1.internal_type not in {VariableDeclaration}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.962. Support: 251.` |
| 38 | `  -1.internal_type not in {CommentLine, Identifier, StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, ?, {}<br>	∧ -1.roles not in {MAP}<br>	∧ -2.diff_line = 0<br>	∧ -2.label not in {<-tab>}<br>	∧ -3.diff_offset ≤ 3<br>	∧ -5.length ≥ 2<br>	∧ +1.reserved not in {(, ,, ;, }}<br>	∧ +1.roles not in {COMMENT}<br>	∧ +2.roles not in {ARGUMENT, COMMENT}<br>	∧ +3.reserved = {<br>	∧ ^1.internal_type not in {VariableDeclaration}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.962. Support: 120.` |
| 39 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, ?, {}<br>	∧ -1.roles not in {MAP}<br>	∧ -2.diff_line = 0<br>	∧ -2.label not in {<-tab>}<br>	∧ +1.reserved not in {(, ,, ;, }}<br>	∧ +1.roles not in {COMMENT}<br>	∧ +2.roles not in {ARGUMENT, COMMENT}<br>	∧ +3.reserved not in {{}<br>	∧ ^1.internal_type = ConditionalExpression<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.973. Support: 1753.` |
| 40 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, ?, {}<br>	∧ -1.roles not in {MAP}<br>	∧ -2.diff_line = 0<br>	∧ -2.label not in {<-tab>}<br>	∧ -3.diff_offset ≥ 5<br>	∧ -3.label in {<newline>}<br>	∧ +1.reserved not in {(, ,, ;, }}<br>	∧ +1.roles not in {COMMENT}<br>	∧ +2.roles not in {ARGUMENT, COMMENT}<br>	∧ +3.reserved not in {{}<br>	∧ ^1.internal_type not in {CallExpression, VariableDeclaration}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.978. Support: 1344.` |
| 41 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, ?, {}<br>	∧ -1.roles not in {MAP}<br>	∧ -2.diff_line = 0<br>	∧ -2.label not in {<-tab>}<br>	∧ -3.diff_offset ≥ 5<br>	∧ -3.label not in {<newline>}<br>	∧ +1.reserved not in {(, ,, ;, }}<br>	∧ +1.roles not in {COMMENT}<br>	∧ +2.roles not in {ARGUMENT, COMMENT}<br>	∧ +3.reserved not in {{}<br>	∧ ^1.internal_type not in {VariableDeclaration}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.986. Support: 37020.` |
| 42 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, ?}<br>	∧ -1.roles not in {MAP}<br>	∧ -2.diff_line = 0<br>	∧ -2.label not in {<-tab>}<br>	∧ -3.diff_offset ≤ 4<br>	∧ +1.reserved not in {(, ,, ;, }}<br>	∧ +1.roles not in {COMMENT}<br>	∧ +1.length ≤ 5<br>	∧ +2.roles not in {ARGUMENT, COMMENT}<br>	∧ +3.reserved = )<br>	∧ ^1.internal_type not in {VariableDeclaration}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.981. Support: 447.` |
| 43 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, ?, {}<br>	∧ -1.roles not in {MAP}<br>	∧ -2.diff_line = 0<br>	∧ -2.label not in {<-tab>}<br>	∧ -3.diff_offset ≤ 4<br>	∧ +1.reserved not in {(, ,, ;, }}<br>	∧ +1.roles not in {COMMENT}<br>	∧ +1.length ≤ 5<br>	∧ +2.roles not in {ARGUMENT, COMMENT}<br>	∧ +3.reserved not in {), {}<br>	∧ ^1.internal_type not in {VariableDeclaration}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.960. Support: 8073.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 11.55813953488372, "max_conf": 0.9999452233314514, "max_support": 37020, "min_conf": 0.9247787594795227, "min_support": 102, "num_rules": 43}}
```
</details>
