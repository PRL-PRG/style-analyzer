# Model report for file:///tmp/top-repos-quality-repos-qhl76jxj/juejin.git HEAD f37ee96d66a8c08ccf65cf985709b53bd1ebcbf1

### Dump

```json
{'created_at': '2021-08-30 05:31:18',
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
 'size': '20.2 kB',
 'tags': [],
 'uuid': 'cee4c4d8-a437-492c-bc63-3810c81c43fa',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-qhl76jxj/juejin.git f37ee96d66a8c08ccf65cf985709b53bd1ebcbf1

# javascript
162 rules, avg.len. 8.1
## train
PPCR: 0.979512
### report
macro
{'f1-score': 0.6148621207266252,
 'precision': 0.6162836549724814,
 'recall': 0.614979782467165,
 'support': 27156}
micro
{'f1-score': 0.9344896155545735,
 'precision': 0.9344896155545735,
 'recall': 0.9344896155545735,
 'support': 27156}
weighted
{'f1-score': 0.9280867840946253,
 'precision': 0.9236406112710397,
 'recall': 0.9344896155545735,
 'support': 27156}
### report_full
macro
{'f1-score': 0.6086833920424717,
 'precision': 0.6162836549724814,
 'recall': 0.6034527097203856,
 'support': 27724}
micro
{'f1-score': 0.9248177842565598,
 'precision': 0.9344896155545735,
 'recall': 0.9153441061895831,
 'support': 27724}
weighted
{'f1-score': 0.9147066450825742,
 'precision': 0.9165175462332342,
 'recall': 0.9153441061895831,
 'support': 27724}
## test
PPCR: 0.981882
### report
macro
{'f1-score': 0.6210938651614234,
 'precision': 0.6284514944043909,
 'recall': 0.6161228712285077,
 'support': 3631}
micro
{'f1-score': 0.9366565684384467,
 'precision': 0.9366565684384467,
 'recall': 0.9366565684384467,
 'support': 3631}
weighted
{'f1-score': 0.9356311923424934,
 'precision': 0.9372558953268323,
 'recall': 0.9366565684384467,
 'support': 3631}
### report_full
macro
{'f1-score': 0.614658992177212,
 'precision': 0.6284514944043909,
 'recall': 0.6055107617875778,
 'support': 3698}
micro
{'f1-score': 0.928093873652613,
 'precision': 0.9366565684384467,
 'recall': 0.9196863169280692,
 'support': 3698}
weighted
{'f1-score': 0.9251966867491086,
 'precision': 0.9352534669765327,
 'recall': 0.9196863169280692,
 'support': 3698}
```

## javascript
### Summary
114 rules, avg.len. 7.7

| | |
|-|-|
|Min support|152|
|Max support|5523|
|Min confidence|0.9207152724266052|
|Max confidence|0.9996469020843506|

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
| 1 | `  ^1.internal_type = MemberExpression<br>⇒ y = ∅<br>Confidence: 0.996. Support: 3929.` |
| 2 | `  -1.label in {<space>}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = '<br>Confidence: 0.968. Support: 203.` |
| 3 | `  +1.reserved = }<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.928. Support: 881.` |
| 4 | `  -1.reserved = {<br>	∧ +1.reserved not in {}}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.934. Support: 826.` |
| 5 | `  -1.internal_type = StringLiteral<br>	∧ -1.reserved not in {{}<br>	∧ +1.reserved not in {}}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = '<br>Confidence: 0.954. Support: 728.` |
| 6 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {{}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.reserved not in {}}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type = VariableDeclaration<br>⇒ y = ␣<br>Confidence: 0.999. Support: 357.` |
| 7 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {{}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.reserved not in {}}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.974. Support: 3535.` |
| 8 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {STRING}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 1.000. Support: 1388.` |
| 9 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = '<br>Confidence: 0.986. Support: 403.` |
| 10 | `  -1.diff_col ≤ 6<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -3.diff_offset ≥ 5<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles in {EXPRESSION} and not in {ASSIGNMENT}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {LITERAL, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.924. Support: 1844.` |
| 11 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = {<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.976. Support: 699.` |
| 12 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {(, {, }}<br>	∧ +1.roles in {COMMENT} and not in {EXPRESSION}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.982. Support: 357.` |
| 13 | `  -1.internal_type = CommentLine<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {(, {, }}<br>	∧ +1.roles not in {COMMENT, EXPRESSION}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ⏎<br>Confidence: 0.951. Support: 172.` |
| 14 | `  ^1.roles in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.994. Support: 3863.` |
| 15 | `  -1.label in {<space>}<br>	∧ ^1.roles in {OPERATOR} and not in {IDENTIFIER}<br>⇒ y = '<br>Confidence: 0.981. Support: 180.` |
| 16 | `  -1.internal_type = Identifier<br>	∧ +1.reserved = =<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 374.` |
| 17 | `  -1.internal_type = Identifier<br>	∧ +1.reserved not in {=}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.957. Support: 3242.` |
| 18 | `  -1.internal_type not in {Identifier}<br>	∧ +1.reserved = }<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.974. Support: 822.` |
| 19 | `  -1.internal_type not in {Identifier}<br>	∧ -1.reserved = {<br>	∧ +1.reserved not in {}}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.936. Support: 830.` |
| 20 | `  -1.internal_type = StringLiteral<br>	∧ -1.reserved not in {{}<br>	∧ +1.reserved not in {}}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = '<br>Confidence: 0.962. Support: 794.` |
| 21 | `  -1.internal_type not in {Identifier, StringLiteral}<br>	∧ -1.reserved = (<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {STRING}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ∅<br>Confidence: 1.000. Support: 1416.` |
| 22 | `  -1.internal_type not in {Identifier, StringLiteral}<br>	∧ -1.label in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = '<br>Confidence: 0.984. Support: 396.` |
| 23 | `  -1.internal_type not in {Identifier, StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles in {EXPRESSION, KEY}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ⏎<br>Confidence: 0.943. Support: 479.` |
| 24 | `  -1.internal_type not in {Identifier, StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ +1.reserved = {<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.972. Support: 655.` |
| 25 | `  -1.internal_type not in {Identifier, StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ +1.reserved not in {(, {, }}<br>	∧ +1.roles in {COMMENT} and not in {EXPRESSION}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.983. Support: 320.` |
| 26 | `  ^1.roles in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 3816.` |
| 27 | `  -1.label in {<space>}<br>	∧ ^1.roles in {OPERATOR} and not in {QUALIFIED}<br>⇒ y = '<br>Confidence: 0.961. Support: 218.` |
| 28 | `  -1.reserved = {<br>	∧ +1.reserved not in {}}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.938. Support: 826.` |
| 29 | `  -1.internal_type = StringLiteral<br>	∧ -1.reserved not in {{}<br>	∧ +1.reserved not in {}}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = '<br>Confidence: 0.964. Support: 742.` |
| 30 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles in {EXPRESSION} and not in {STRING}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 945.` |
| 31 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = '<br>Confidence: 0.992. Support: 416.` |
| 32 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -2.diff_offset ≤ 7<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.roles in {LITERAL} and not in {OPERATOR, QUALIFIED}<br>⇒ y = ⏎<br>Confidence: 0.980. Support: 465.` |
| 33 | `  -1.diff_col ≤ 6<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles in {EXPRESSION} and not in {ASSIGNMENT}<br>	∧ ^1.roles not in {LITERAL, OPERATOR, QUALIFIED, STATEMENT}<br>⇒ y = ␣<br>Confidence: 0.928. Support: 1639.` |
| 34 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {{}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>	∧ ^2.internal_type = VariableDeclaration<br>⇒ y = ␣<br>Confidence: 0.982. Support: 468.` |
| 35 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {{}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.953. Support: 3592.` |
| 36 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = {<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 213.` |
| 37 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = {<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ␣<br>Confidence: 0.979. Support: 634.` |
| 38 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {{}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {(, {, }}<br>	∧ +1.roles in {COMMENT} and not in {EXPRESSION}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.983. Support: 387.` |
| 39 | `  -1.internal_type = CommentLine<br>	∧ -1.reserved not in {{}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {(, {, }}<br>	∧ +1.roles not in {COMMENT, EXPRESSION}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ⏎<br>Confidence: 0.928. Support: 174.` |
| 40 | `  -1.reserved = {<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.940. Support: 843.` |
| 41 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles in {EXPRESSION, KEY}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ⏎<br>Confidence: 0.951. Support: 495.` |
| 42 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {{}<br>	∧ +1.reserved = =<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.992. Support: 432.` |
| 43 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ +1.reserved = {<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 203.` |
| 44 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ +1.reserved = {<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.982. Support: 626.` |
| 45 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {{}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles in {COMMENT} and not in {EXPRESSION}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.987. Support: 339.` |
| 46 | `  -1.internal_type = CommentLine<br>	∧ -1.reserved not in {{}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles not in {COMMENT, EXPRESSION}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ⏎<br>Confidence: 0.935. Support: 176.` |
| 47 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = if<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.970. Support: 217.` |
| 48 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {if, {}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.933. Support: 5523.` |
| 49 | `  -1.reserved = {<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.940. Support: 826.` |
| 50 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 900.` |
| 51 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles in {COMMENT}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.990. Support: 349.` |
| 52 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -2.label in {<space>}<br>	∧ -2.length ≤ 2<br>	∧ -3.roles in {IDENTIFIER}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {COMMENT}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 315.` |
| 53 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {{}<br>	∧ +1.reserved not in {}}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>	∧ ^2.internal_type = VariableDeclaration<br>⇒ y = ␣<br>Confidence: 0.958. Support: 492.` |
| 54 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {{}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.reserved not in {}}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.975. Support: 3524.` |
| 55 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {{}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = )<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.994. Support: 589.` |
| 56 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {{}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = ;<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 485.` |
| 57 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {{}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = ,<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 329.` |
| 58 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {), ,, ;, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.948. Support: 353.` |
| 59 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {), ,, ;, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = '<br>Confidence: 0.948. Support: 182.` |
| 60 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.diff_offset ≥ 4<br>	∧ +1.reserved not in {), ,, ;, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ␣<br>Confidence: 0.958. Support: 1052.` |
| 61 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.diff_offset ≤ 3<br>	∧ +1.reserved = {<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 324.` |
| 62 | `  -1.reserved = {<br>	∧ +1.reserved not in {}}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.929. Support: 796.` |
| 63 | `  -1.reserved = (<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {STRING}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 858.` |
| 64 | `  -1.reserved not in {(, {}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles in {COMMENT}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.973. Support: 355.` |
| 65 | `  -1.reserved not in {(, {}<br>	∧ -2.length ≤ 2<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {COMMENT, LITERAL}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>	∧ ^2.internal_type = VariableDeclaration<br>⇒ y = ␣<br>Confidence: 0.998. Support: 227.` |
| 66 | `  •••start_line ≤ 250<br>	∧ -1.reserved not in {(, {}<br>	∧ -2.length ≤ 2<br>	∧ -3.diff_line = 0<br>	∧ -3.length ≥ 3<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {COMMENT, LITERAL}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ⏎<br>Confidence: 0.934. Support: 631.` |
| 67 | `  -1.internal_type = StringLiteral<br>	∧ +1.reserved not in {}}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = '<br>Confidence: 0.960. Support: 713.` |
| 68 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type = VariableDeclarator<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.956. Support: 493.` |
| 69 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.reserved not in {}}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.979. Support: 3514.` |
| 70 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label in {<space>}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {}}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = '<br>Confidence: 0.978. Support: 201.` |
| 71 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = )<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.994. Support: 595.` |
| 72 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = ;<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 480.` |
| 73 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = (<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {), ;, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.950. Support: 350.` |
| 74 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = ,<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 326.` |
| 75 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.diff_offset ≥ 4<br>	∧ +1.reserved not in {), ,, ;, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.959. Support: 1098.` |
| 76 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.diff_offset ≤ 3<br>	∧ +1.reserved = {<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.990. Support: 339.` |
| 77 | `  -1.internal_type = Identifier<br>	∧ ^1.internal_type = VariableDeclarator<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 363.` |
| 78 | `  -1.internal_type = Identifier<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.951. Support: 3237.` |
| 79 | `  -1.internal_type = CommentLine<br>	∧ -1.reserved not in {(, {}<br>	∧ +1.reserved not in {(, {, }}<br>	∧ +1.roles not in {COMMENT, EXPRESSION}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ⏎<br>Confidence: 0.943. Support: 166.` |
| 80 | `  -1.roles in {IDENTIFIER}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>	∧ ^2.internal_type = VariableDeclaration<br>⇒ y = ␣<br>Confidence: 0.999. Support: 364.` |
| 81 | `  -1.roles in {IDENTIFIER}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.956. Support: 3241.` |
| 82 | `  -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved = }<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.969. Support: 801.` |
| 83 | `  -1.reserved = {<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved not in {}}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.928. Support: 839.` |
| 84 | `  -1.internal_type = StringLiteral<br>	∧ -1.reserved not in {{}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved not in {}}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = '<br>Confidence: 0.967. Support: 766.` |
| 85 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 1376.` |
| 86 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = '<br>Confidence: 0.987. Support: 430.` |
| 87 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles in {EXPRESSION, KEY}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ⏎<br>Confidence: 0.959. Support: 498.` |
| 88 | `  -1.diff_col ≤ 6<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ -2.diff_offset ≥ 4<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles in {EXPRESSION} and not in {ASSIGNMENT, KEY}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.921. Support: 1482.` |
| 89 | `  -1.diff_col ≤ 6<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = =<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ -2.diff_offset ≤ 3<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles in {EXPRESSION} and not in {ASSIGNMENT, KEY}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 328.` |
| 90 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved = {<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.980. Support: 689.` |
| 91 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved not in {(, {, }}<br>	∧ +1.roles in {COMMENT} and not in {EXPRESSION}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.975. Support: 338.` |
| 92 | `  -1.internal_type = CommentLine<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved not in {(, {, }}<br>	∧ +1.roles not in {COMMENT, EXPRESSION}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ⏎<br>Confidence: 0.930. Support: 193.` |
| 93 | `  -1.reserved = {<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.941. Support: 835.` |
| 94 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {STRING}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 889.` |
| 95 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {COMMENT}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type = ObjectExpression<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ⏎<br>Confidence: 0.962. Support: 509.` |
| 96 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {COMMENT}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression, ObjectExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = '<br>Confidence: 0.987. Support: 267.` |
| 97 | `  -1.internal_type = CommentLine<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {COMMENT}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression, ObjectExpression}<br>	∧ ^1.roles in {FILE} and not in {BLOCK, OPERATOR}<br>⇒ y = ⏎<br>Confidence: 0.924. Support: 242.` |
| 98 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {{}<br>	∧ +1.reserved not in {}}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type = VariableDeclarator<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.972. Support: 479.` |
| 99 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {{}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.reserved not in {}}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression, VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.974. Support: 3604.` |
| 100 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = {<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression, VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 193.` |
| 101 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = {<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression, VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.988. Support: 635.` |
| 102 | `  -1.diff_col ≤ 1<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label in {<space>}<br>	∧ -1.reserved not in {{}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.roles in {LITERAL}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression, VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = '<br>Confidence: 0.990. Support: 152.` |
| 103 | `  -1.diff_col ≤ 1<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {{}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.roles not in {LITERAL}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression, ObjectProperty, VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.943. Support: 1785.` |
| 104 | `  -1.internal_type = Identifier<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>	∧ ^2.internal_type = VariableDeclaration<br>⇒ y = ␣<br>Confidence: 0.999. Support: 359.` |
| 105 | `  -1.internal_type = Identifier<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.953. Support: 3205.` |
| 106 | `  -1.internal_type not in {Identifier}<br>	∧ +1.reserved = }<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.976. Support: 856.` |
| 107 | `  -1.internal_type not in {Identifier}<br>	∧ -1.reserved = {<br>	∧ +1.reserved not in {}}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.946. Support: 827.` |
| 108 | `  -1.internal_type not in {Identifier, StringLiteral}<br>	∧ -1.reserved = (<br>	∧ +1.internal_type = StringLiteral<br>	∧ +1.reserved not in {}}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = '<br>Confidence: 0.927. Support: 186.` |
| 109 | `  -1.internal_type not in {Identifier, StringLiteral}<br>	∧ -1.reserved = (<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 1337.` |
| 110 | `  -1.internal_type not in {Identifier, StringLiteral}<br>	∧ -1.label in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = '<br>Confidence: 0.990. Support: 466.` |
| 111 | `  -1.internal_type not in {Identifier, StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles in {EXPRESSION, KEY}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ⏎<br>Confidence: 0.959. Support: 452.` |
| 112 | `  -1.diff_col ≤ 6<br>	∧ -1.internal_type not in {Identifier, StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -2.diff_offset ≥ 4<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles in {EXPRESSION} and not in {ASSIGNMENT, KEY}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.922. Support: 1553.` |
| 113 | `  -1.internal_type not in {Identifier, StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ +1.reserved = {<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.977. Support: 683.` |
| 114 | `  -1.internal_type not in {Identifier, StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ +1.reserved not in {(, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.roles not in {MAP}<br>⇒ y = ∅<br>Confidence: 0.941. Support: 1758.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 7.657894736842105, "max_conf": 0.9996469020843506, "max_support": 5523, "min_conf": 0.9207152724266052, "min_support": 152, "num_rules": 114}}
```
</details>
