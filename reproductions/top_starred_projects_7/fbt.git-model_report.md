# Model report for file:///tmp/top-repos-quality-repos-z4j29f5i/fbt.git HEAD f8ea3fecfccd4fc07fee509b1dd5afc7bbe8f155

### Dump

```json
{'created_at': '2021-08-31 13:08:37',
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
 'size': '23.9 kB',
 'tags': [],
 'uuid': '25b50afd-cf74-4f49-a294-82c5847b5d01',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-z4j29f5i/fbt.git f8ea3fecfccd4fc07fee509b1dd5afc7bbe8f155

# javascript
238 rules, avg.len. 9.7
## train
PPCR: 0.984933
### report
macro
{'f1-score': 0.6937052574061956,
 'precision': 0.7330053227507873,
 'recall': 0.6677223845013298,
 'support': 161338}
micro
{'f1-score': 0.9557140909147256,
 'precision': 0.9557140909147256,
 'recall': 0.9557140909147256,
 'support': 161338}
weighted
{'f1-score': 0.9534244700417641,
 'precision': 0.9529870848903417,
 'recall': 0.9557140909147256,
 'support': 161338}
### report_full
macro
{'f1-score': 0.6801422850366904,
 'precision': 0.7330053227507873,
 'recall': 0.6475130198563507,
 'support': 163806}
micro
{'f1-score': 0.9484597593681569,
 'precision': 0.9557140909147256,
 'recall': 0.9413147259563142,
 'support': 163806}
weighted
{'f1-score': 0.9453134399251855,
 'precision': 0.9523958950810285,
 'recall': 0.9413147259563142,
 'support': 163806}
## test
PPCR: 0.985972
### report
macro
{'f1-score': 0.6587158664482873,
 'precision': 0.7043503390567499,
 'recall': 0.6298662519631166,
 'support': 27130}
micro
{'f1-score': 0.9530777736822705,
 'precision': 0.9530777736822705,
 'recall': 0.9530777736822705,
 'support': 27130}
weighted
{'f1-score': 0.9505723696118906,
 'precision': 0.9504100374441219,
 'recall': 0.9530777736822705,
 'support': 27130}
### report_full
macro
{'f1-score': 0.6486536130162028,
 'precision': 0.7043503390567499,
 'recall': 0.6131227879007787,
 'support': 27516}
micro
{'f1-score': 0.9463455696665812,
 'precision': 0.9530777736822705,
 'recall': 0.9397078063672045,
 'support': 27516}
weighted
{'f1-score': 0.9433279244017422,
 'precision': 0.949757483804646,
 'recall': 0.9397078063672045,
 'support': 27516}
```

## javascript
### Summary
137 rules, avg.len. 9.7

| | |
|-|-|
|Min support|135|
|Max support|36780|
|Min confidence|0.9212328791618347|
|Max confidence|0.9993215799331665|

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
| 1 | `  •••start_col ≥ 8<br>	∧ -1.reserved = ,<br>	∧ +1.roles in {EXPRESSION}<br>	∧ +4.reserved = ,<br>	∧ ^1.roles in {MAP}<br>⇒ y = ⏎<br>Confidence: 0.997. Support: 32722.` |
| 2 | `  •••start_col ≥ 8<br>	∧ -1.reserved = ,<br>	∧ -3.length ≥ 7<br>	∧ +1.roles in {EXPRESSION}<br>	∧ +4.reserved not in {,}<br>	∧ ^1.roles in {MAP}<br>⇒ y = ⏎<br>Confidence: 0.990. Support: 455.` |
| 3 | `  -1.reserved not in {,}<br>	∧ -1.roles in {STRING}<br>	∧ -3.reserved = ,<br>	∧ +5.reserved = :<br>⇒ y = "<br>Confidence: 0.984. Support: 612.` |
| 4 | `  -1.reserved not in {,, {}<br>	∧ -1.roles not in {STRING}<br>	∧ -3.reserved = ,<br>⇒ y = ∅<br>Confidence: 0.993. Support: 36729.` |
| 5 | `  -1.reserved = :<br>⇒ y = ␣<br>Confidence: 0.974. Support: 4258.` |
| 6 | `  -1.reserved not in {,, :}<br>	∧ -3.reserved not in {,}<br>	∧ -3.roles in {VALUE}<br>	∧ +1.internal_type = StringLiteral<br>⇒ y = "<br>Confidence: 0.952. Support: 676.` |
| 7 | `  -1.reserved not in {,, :}<br>	∧ -2.label in {<space>}<br>	∧ -3.diff_offset ≥ 4<br>	∧ -3.reserved not in {,}<br>	∧ +1.internal_type = StringLiteral<br>⇒ y = ␣<br>Confidence: 0.972. Support: 269.` |
| 8 | `  -1.reserved not in {,, :}<br>	∧ -2.label not in {<space>}<br>	∧ -3.reserved not in {,}<br>	∧ -3.roles not in {VALUE}<br>	∧ +1.internal_type = StringLiteral<br>	∧ +3.internal_type not in {StringLiteral}<br>⇒ y = '<br>Confidence: 0.936. Support: 2801.` |
| 9 | `  -1.internal_type = StringLiteral<br>	∧ -1.reserved not in {,, :}<br>	∧ -3.reserved not in {,}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +2.internal_type = StringLiteral<br>	∧ ^2.roles in {DECLARATION}<br>⇒ y = "<br>Confidence: 0.984. Support: 221.` |
| 10 | `  -1.internal_type = StringLiteral<br>	∧ -1.reserved not in {,, :}<br>	∧ -3.reserved not in {,}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +2.internal_type not in {StringLiteral}<br>⇒ y = '<br>Confidence: 0.942. Support: 2306.` |
| 11 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ;<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = }<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.972. Support: 937.` |
| 12 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +2.reserved = }<br>⇒ y = ∅<br>Confidence: 0.952. Support: 196.` |
| 13 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ -2.label in {<space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +2.reserved not in {}}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.942. Support: 1999.` |
| 14 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, {}<br>	∧ -3.reserved not in {,}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +2.roles in {EXPRESSION}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.950. Support: 1340.` |
| 15 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, {}<br>	∧ -3.reserved not in {,}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles in {RIGHT}<br>	∧ +2.roles not in {EXPRESSION}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.991. Support: 690.` |
| 16 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, {}<br>	∧ -3.reserved not in {,}<br>	∧ -3.roles in {BINARY}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +2.roles not in {EXPRESSION}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.957. Support: 220.` |
| 17 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, {}<br>	∧ -3.reserved not in {,}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = {<br>	∧ +3.reserved not in {:}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.956. Support: 1388.` |
| 18 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = const<br>	∧ +1.internal_type not in {StringLiteral}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 737.` |
| 19 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = return<br>	∧ +1.internal_type not in {StringLiteral}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 457.` |
| 20 | `  -1.internal_type = CommentLine<br>	∧ -1.reserved not in {,, :, ;, return, {}<br>	∧ -3.reserved not in {,}<br>	∧ +1.reserved not in {{}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ⏎<br>Confidence: 0.958. Support: 416.` |
| 21 | `  •••start_line ≤ 10<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, {}<br>	∧ -2.roles in {COMMENT}<br>	∧ -3.reserved not in {,}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {{}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ⏎⏎<br>Confidence: 0.953. Support: 204.` |
| 22 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved = if<br>	∧ +1.internal_type not in {StringLiteral}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 247.` |
| 23 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, :, ;}<br>	∧ -3.reserved not in {,}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = =<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>	∧ ^2.roles in {BLOCK}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 153.` |
| 24 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, ;, {}<br>	∧ -3.reserved not in {,}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = =<br>⇒ y = ␣<br>Confidence: 0.999. Support: 338.` |
| 25 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, {}<br>	∧ -2.label in {<-space>}<br>	∧ -3.reserved not in {,}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.965. Support: 554.` |
| 26 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved = import<br>	∧ -2.label not in {<-space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 150.` |
| 27 | `  -1.internal_type = DirectiveLiteral<br>	∧ -1.reserved not in {,, :, ;, {}<br>	∧ -2.label not in {<-space>}<br>	∧ -3.reserved not in {,}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {{}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = '<br>Confidence: 0.996. Support: 137.` |
| 28 | `  -1.internal_type not in {CommentLine, DirectiveLiteral, StringLiteral}<br>	∧ -1.reserved not in {,, :, ;}<br>	∧ -2.label not in {<-space>}<br>	∧ -3.reserved not in {,}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {{}<br>	∧ +2.internal_type = TemplateElement<br>	∧ ^1.roles in {MAP} and not in {OPERATOR}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.967. Support: 166.` |
| 29 | `  -1.diff_col ≥ 3<br>	∧ -1.internal_type not in {CommentLine, DirectiveLiteral, StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, {}<br>	∧ -1.length ≥ 8<br>	∧ -3.reserved not in {,}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.928. Support: 466.` |
| 30 | `  -1.diff_col ≤ 2<br>	∧ -1.internal_type not in {CommentLine, DirectiveLiteral, StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, {}<br>	∧ -3.reserved not in {,}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.932. Support: 6132.` |
| 31 | `  -1.internal_type not in {CommentLine, DirectiveLiteral, StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, {}<br>	∧ -3.reserved not in {,}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.980. Support: 17518.` |
| 32 | `  -1.reserved = ,<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +2.length ≤ 4<br>	∧ ^1.internal_type not in {CallExpression}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.931. Support: 1822.` |
| 33 | `  -1.reserved not in {,}<br>	∧ -1.roles in {STRING}<br>	∧ -3.reserved = ,<br>	∧ -4.roles in {VALUE}<br>⇒ y = "<br>Confidence: 0.960. Support: 613.` |
| 34 | `  -1.reserved not in {,, :}<br>	∧ -2.label in {<space>}<br>	∧ -3.diff_col ≥ 4<br>	∧ -3.reserved not in {,}<br>	∧ +1.internal_type = StringLiteral<br>⇒ y = ␣<br>Confidence: 0.976. Support: 274.` |
| 35 | `  -1.internal_type = StringLiteral<br>	∧ -1.reserved not in {,, :}<br>	∧ -2.diff_col ≥ 22<br>	∧ -3.reserved not in {,}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +2.internal_type = StringLiteral<br>⇒ y = "<br>Confidence: 0.934. Support: 235.` |
| 36 | `  •••start_col ≤ 14<br>	∧ •••start_line ≤ 26<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ;<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>⇒ y = ⏎⏎<br>Confidence: 0.969. Support: 271.` |
| 37 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;}<br>	∧ -3.reserved not in {,}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +2.roles in {EXPRESSION}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.955. Support: 1426.` |
| 38 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;}<br>	∧ -3.reserved not in {,}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles in {RIGHT}<br>	∧ +2.roles not in {EXPRESSION}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.995. Support: 744.` |
| 39 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;}<br>	∧ -3.reserved not in {,}<br>	∧ -3.roles in {BINARY}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +2.roles not in {EXPRESSION}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.972. Support: 232.` |
| 40 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +2.reserved = }<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.937. Support: 199.` |
| 41 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, {}<br>	∧ -3.label not in {<space>}<br>	∧ -3.reserved not in {,}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^2.internal_type = VariableDeclaration<br>⇒ y = ␣<br>Confidence: 0.942. Support: 1965.` |
| 42 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, {}<br>	∧ -3.reserved not in {,}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = {<br>	∧ +2.roles not in {MAP}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.980. Support: 1244.` |
| 43 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, {}<br>	∧ -2.label in {<-space>}<br>	∧ -3.reserved not in {,}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.972. Support: 548.` |
| 44 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved = if<br>	∧ -2.label not in {<-space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 207.` |
| 45 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, ;, return, {}<br>	∧ -2.label not in {<-space>}<br>	∧ -3.reserved not in {,}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = =<br>⇒ y = ␣<br>Confidence: 0.998. Support: 286.` |
| 46 | `  -1.diff_col ≥ 3<br>	∧ -1.diff_offset ≥ 11<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, {}<br>	∧ -3.reserved not in {,}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.959. Support: 404.` |
| 47 | `  -1.diff_col ≥ 3<br>	∧ -1.diff_offset ≤ 10<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, ;, {}<br>	∧ -2.label not in {<-space>}<br>	∧ -3.reserved not in {,}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles in {IDENTIFIER}<br>	∧ +1.length ≥ 2<br>⇒ y = ␣<br>Confidence: 0.942. Support: 539.` |
| 48 | `  •••start_col ≤ 14<br>	∧ -1.diff_col ≤ 2<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, {}<br>	∧ -2.label in {<space>}<br>	∧ -3.reserved not in {,}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {}<br>	∧ +1.roles not in {CALLEE}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>	∧ ^2.roles not in {MODULE}<br>⇒ y = ∅<br>Confidence: 0.931. Support: 238.` |
| 49 | `  -1.diff_col ≤ 2<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, {}<br>	∧ -2.label not in {<space>}<br>	∧ -3.reserved not in {,}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {}<br>	∧ +1.roles not in {CALLEE}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>	∧ ^2.roles not in {MODULE}<br>⇒ y = ∅<br>Confidence: 0.947. Support: 5915.` |
| 50 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, import}<br>	∧ -2.label not in {<-space>}<br>	∧ -3.label in {<space>}<br>	∧ -3.reserved not in {,}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {{}<br>	∧ +1.length ≤ 1<br>	∧ +2.internal_type = TemplateElement<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.964. Support: 151.` |
| 51 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, {}<br>	∧ -3.reserved not in {,}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.972. Support: 17977.` |
| 52 | `  -1.reserved = ,<br>	∧ -2.roles in {ARGUMENT}<br>	∧ -3.length ≤ 2<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.roles not in {MAP}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 455.` |
| 53 | `  -1.reserved = ,<br>	∧ -2.roles not in {ARGUMENT}<br>	∧ -3.length ≤ 2<br>	∧ +1.roles in {EXPRESSION, NUMBER}<br>	∧ ^1.roles not in {MAP}<br>⇒ y = ␣<br>Confidence: 0.983. Support: 208.` |
| 54 | `  -1.reserved not in {,, :}<br>	∧ -2.label in {<space>}<br>	∧ -3.diff_col ≥ 4<br>	∧ -3.reserved not in {,}<br>	∧ -3.length ≤ 18<br>	∧ +1.internal_type = StringLiteral<br>⇒ y = ␣<br>Confidence: 0.957. Support: 246.` |
| 55 | `  -1.reserved not in {,, :}<br>	∧ -2.label not in {<space>}<br>	∧ -3.reserved not in {,}<br>	∧ -3.length ≤ 18<br>	∧ +1.internal_type = StringLiteral<br>	∧ +3.roles not in {LITERAL}<br>⇒ y = '<br>Confidence: 0.935. Support: 2702.` |
| 56 | `  -1.internal_type = StringLiteral<br>	∧ -1.reserved not in {,, :}<br>	∧ -3.reserved not in {,}<br>	∧ -4.roles not in {LITERAL}<br>	∧ +1.internal_type not in {StringLiteral}<br>⇒ y = '<br>Confidence: 0.927. Support: 2497.` |
| 57 | `  -1.internal_type = CommentLine<br>	∧ -1.reserved not in {,, :, ;, const, {}<br>	∧ -3.reserved not in {,}<br>	∧ +1.reserved not in {{}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ⏎<br>Confidence: 0.956. Support: 471.` |
| 58 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved = return<br>	∧ +1.internal_type not in {StringLiteral}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 429.` |
| 59 | `  •••start_line ≤ 11<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, {}<br>	∧ -2.roles in {COMMENT}<br>	∧ -3.reserved not in {,}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {{}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ⏎⏎<br>Confidence: 0.925. Support: 179.` |
| 60 | `  -1.diff_col ≥ 3<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, {}<br>	∧ -1.length ≥ 11<br>	∧ -3.reserved not in {,}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.938. Support: 398.` |
| 61 | `  -1.diff_col ≥ 3<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, ;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≤ 10<br>	∧ -2.label not in {<-space>}<br>	∧ -3.reserved not in {,}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>⇒ y = ␣<br>Confidence: 0.942. Support: 795.` |
| 62 | `  •••start_col ≤ 14<br>	∧ -1.diff_col ≤ 2<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, {}<br>	∧ -2.label in {<space>}<br>	∧ -3.reserved not in {,}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>	∧ ^2.roles not in {MODULE}<br>⇒ y = ∅<br>Confidence: 0.946. Support: 267.` |
| 63 | `  -1.diff_col ≤ 2<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved = (<br>	∧ -2.label not in {<space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=}<br>	∧ +1.length ≥ 2<br>	∧ +2.reserved = )<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>	∧ ^2.roles not in {MODULE}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 397.` |
| 64 | `  -1.diff_col ≤ 2<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {(, ,, :, ;, {}<br>	∧ -2.label not in {<space>}<br>	∧ -3.reserved not in {,}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>	∧ ^2.roles not in {MODULE}<br>⇒ y = ∅<br>Confidence: 0.965. Support: 5019.` |
| 65 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, {}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ -3.reserved not in {,}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.993. Support: 11566.` |
| 66 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -3.reserved not in {,}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.960. Support: 6081.` |
| 67 | `  -1.reserved not in {,}<br>	∧ -1.roles in {STRING}<br>	∧ -3.reserved = ,<br>	∧ -4.roles in {MAP}<br>⇒ y = "<br>Confidence: 0.938. Support: 664.` |
| 68 | `  -1.reserved not in {,}<br>	∧ -1.roles not in {STRING}<br>	∧ -3.reserved = ,<br>	∧ +1.internal_type not in {Identifier}<br>⇒ y = ∅<br>Confidence: 0.994. Support: 36780.` |
| 69 | `  •••start_col ≤ 14<br>	∧ •••start_line ≤ 30<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ;<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>⇒ y = ⏎⏎<br>Confidence: 0.940. Support: 241.` |
| 70 | `  •••start_col ≥ 9<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, ;, return, {}<br>	∧ -3.reserved not in {,}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = =<br>⇒ y = ␣<br>Confidence: 0.998. Support: 306.` |
| 71 | `  •••start_col ≥ 9<br>	∧ -1.diff_col ≥ 3<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, {}<br>	∧ -2.diff_offset ≥ 24<br>	∧ -3.reserved not in {,}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.993. Support: 338.` |
| 72 | `  •••start_col ≥ 9<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, {}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ -3.reserved not in {,}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.993. Support: 10932.` |
| 73 | `  •••start_col ≥ 9<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -3.reserved not in {,}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.958. Support: 6023.` |
| 74 | `  •••start_col ≤ 8<br>	∧ -1.diff_col ≥ 3<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, {}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ -3.reserved not in {,}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {{, }}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.968. Support: 294.` |
| 75 | `  •••start_col ≤ 8<br>	∧ -1.diff_col ≥ 3<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, ;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.label in {<newline>}<br>	∧ -3.reserved not in {,}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>⇒ y = ␣<br>Confidence: 0.996. Support: 383.` |
| 76 | `  •••start_col ≤ 8<br>	∧ -1.diff_col ≤ 2<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved = }<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {OPERATOR, STATEMENT}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ⏎⏎<br>Confidence: 0.926. Support: 237.` |
| 77 | `  •••start_col ≤ 8<br>	∧ -1.diff_col ≤ 2<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved = }<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.984. Support: 480.` |
| 78 | `  •••start_col ≤ 8<br>	∧ -1.diff_col ≤ 2<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, :, ;}<br>	∧ -1.roles in {CALL}<br>	∧ -3.reserved not in {,}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = (<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.996. Support: 135.` |
| 79 | `  •••start_col ≤ 8<br>	∧ -1.diff_col ≤ 2<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, ;, {}<br>	∧ -1.roles not in {CALL}<br>	∧ -3.reserved not in {,}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = (<br>⇒ y = ␣<br>Confidence: 0.974. Support: 135.` |
| 80 | `  •••start_col ≤ 8<br>	∧ -1.diff_col ≤ 2<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, {, }}<br>	∧ -3.reserved not in {,}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {{, }}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.984. Support: 1767.` |
| 81 | `  •••start_col ≥ 8<br>	∧ -1.reserved = ,<br>	∧ -5.diff_col ≤ 38<br>	∧ +1.roles in {EXPRESSION}<br>	∧ +4.reserved = ,<br>	∧ ^1.roles in {MAP}<br>⇒ y = ⏎<br>Confidence: 0.999. Support: 32745.` |
| 82 | `  •••start_col ≥ 8<br>	∧ -1.reserved = ,<br>	∧ -3.length ≥ 7<br>	∧ -5.diff_col ≤ 38<br>	∧ +1.roles in {EXPRESSION}<br>	∧ +4.reserved not in {,}<br>	∧ ^1.roles in {MAP}<br>⇒ y = ⏎<br>Confidence: 0.990. Support: 443.` |
| 83 | `  -1.reserved = ,<br>	∧ -2.roles not in {ARGUMENT}<br>	∧ -3.length ≤ 2<br>	∧ +1.internal_type = NumericLiteral<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.roles not in {MAP}<br>⇒ y = ␣<br>Confidence: 0.975. Support: 177.` |
| 84 | `  -1.reserved not in {,, :}<br>	∧ -3.reserved not in {,}<br>	∧ -5.reserved = :<br>	∧ +1.internal_type = StringLiteral<br>⇒ y = "<br>Confidence: 0.941. Support: 664.` |
| 85 | `  -1.reserved not in {,, :}<br>	∧ -2.label not in {<space>}<br>	∧ -3.reserved not in {,}<br>	∧ -5.reserved not in {:}<br>	∧ +1.internal_type = StringLiteral<br>	∧ +3.internal_type not in {StringLiteral}<br>⇒ y = '<br>Confidence: 0.934. Support: 2737.` |
| 86 | `  -1.internal_type = StringLiteral<br>	∧ -1.reserved not in {,, :}<br>	∧ -3.reserved not in {,}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +2.roles in {STRING}<br>	∧ ^2.roles in {DECLARATION}<br>⇒ y = "<br>Confidence: 0.977. Support: 193.` |
| 87 | `  -1.internal_type = StringLiteral<br>	∧ -1.reserved not in {,, :}<br>	∧ -3.reserved not in {,}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +2.roles not in {STRING}<br>⇒ y = '<br>Confidence: 0.942. Support: 2360.` |
| 88 | `  •••start_col ≤ 14<br>	∧ •••start_line ≤ 33<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ;<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>⇒ y = ⏎⏎<br>Confidence: 0.950. Support: 270.` |
| 89 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -3.reserved not in {,}<br>	∧ -3.roles in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +2.roles not in {EXPRESSION}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.982. Support: 992.` |
| 90 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, {}<br>	∧ -3.reserved not in {,}<br>	∧ -3.length ≥ 2<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.internal_type = VariableDeclarator<br>⇒ y = ␣<br>Confidence: 0.968. Support: 1709.` |
| 91 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, {}<br>	∧ -3.reserved not in {,}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = {<br>	∧ +3.reserved not in {:}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>⇒ y = ␣<br>Confidence: 0.958. Support: 1333.` |
| 92 | `  -1.internal_type = CommentLine<br>	∧ -1.reserved not in {,, :, ;, const, {}<br>	∧ -3.reserved not in {,}<br>	∧ +1.reserved not in {{}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ⏎<br>Confidence: 0.959. Support: 478.` |
| 93 | `  •••start_line ≤ 10<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, {}<br>	∧ -2.roles in {COMMENT}<br>	∧ -3.reserved not in {,}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {{}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ⏎⏎<br>Confidence: 0.924. Support: 190.` |
| 94 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, {}<br>	∧ -2.label in {<-space>}<br>	∧ -3.reserved not in {,}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.974. Support: 556.` |
| 95 | `  -1.diff_col ≥ 3<br>	∧ -1.diff_offset ≥ 11<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, {}<br>	∧ -3.reserved not in {,}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.942. Support: 390.` |
| 96 | `  -1.diff_col ≥ 3<br>	∧ -1.diff_offset ≤ 10<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, ;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.label not in {<-space>}<br>	∧ -3.diff_offset ≤ 27<br>	∧ -3.reserved not in {,}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>⇒ y = ␣<br>Confidence: 0.940. Support: 821.` |
| 97 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, {}<br>	∧ -3.reserved not in {,}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.970. Support: 18258.` |
| 98 | `  -1.reserved not in {,}<br>	∧ -1.roles in {STRING}<br>	∧ -3.reserved = ,<br>	∧ -4.length ≥ 20<br>⇒ y = "<br>Confidence: 0.962. Support: 637.` |
| 99 | `  -1.reserved not in {,}<br>	∧ -1.roles not in {STRING}<br>	∧ -3.reserved = ,<br>	∧ +1.roles not in {MAP}<br>⇒ y = ∅<br>Confidence: 0.992. Support: 36492.` |
| 100 | `  -1.reserved not in {,, :}<br>	∧ -2.label not in {<space>}<br>	∧ -3.reserved not in {,}<br>	∧ -5.reserved not in {:}<br>	∧ +1.internal_type = StringLiteral<br>	∧ +3.roles not in {STRING}<br>⇒ y = '<br>Confidence: 0.925. Support: 2767.` |
| 101 | `  -1.internal_type = StringLiteral<br>	∧ -1.reserved not in {,, :}<br>	∧ -3.reserved not in {,}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^2.internal_type not in {VariableDeclarator}<br>⇒ y = '<br>Confidence: 0.928. Support: 2472.` |
| 102 | `  •••start_col ≤ 20<br>	∧ •••start_line ≤ 33<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ;<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>⇒ y = ⏎⏎<br>Confidence: 0.938. Support: 282.` |
| 103 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, {}<br>	∧ -3.reserved not in {,}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.internal_type = VariableDeclarator<br>⇒ y = ␣<br>Confidence: 0.923. Support: 2066.` |
| 104 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = {<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.954. Support: 140.` |
| 105 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, :, ;}<br>	∧ -3.reserved not in {,}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = =<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.roles in {BLOCK}<br>⇒ y = ∅<br>Confidence: 0.977. Support: 151.` |
| 106 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, ;, {}<br>	∧ -2.label not in {<-space>}<br>	∧ -3.reserved not in {,}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = =<br>⇒ y = ␣<br>Confidence: 0.999. Support: 339.` |
| 107 | `  -1.diff_col ≥ 3<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, {}<br>	∧ -2.diff_offset ≥ 19<br>	∧ -3.reserved not in {,}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.978. Support: 425.` |
| 108 | `  -1.diff_col ≥ 3<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, ;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.diff_offset ≤ 18<br>	∧ -2.label not in {<-space>}<br>	∧ -3.diff_offset ≤ 33<br>	∧ -3.reserved not in {,}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>⇒ y = ␣<br>Confidence: 0.938. Support: 800.` |
| 109 | `  -1.internal_type = StringLiteral<br>	∧ -1.reserved not in {,, :}<br>	∧ -3.reserved not in {,}<br>	∧ -4.internal_type not in {StringLiteral}<br>	∧ +1.internal_type not in {StringLiteral}<br>⇒ y = '<br>Confidence: 0.927. Support: 2487.` |
| 110 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, {}<br>	∧ -3.reserved not in {,}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = {<br>	∧ +2.roles not in {KEY}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>⇒ y = ␣<br>Confidence: 0.953. Support: 1313.` |
| 111 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, import}<br>	∧ -2.label not in {<-space>}<br>	∧ -3.label in {<space>}<br>	∧ -3.reserved not in {,}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {{}<br>	∧ +3.reserved = `<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.921. Support: 146.` |
| 112 | `  -1.diff_col ≥ 3<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, ;, {}<br>	∧ -1.length ≤ 6<br>	∧ -2.label not in {<-space>}<br>	∧ -3.reserved not in {,}<br>	∧ -5.diff_offset ≤ 17<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>⇒ y = ␣<br>Confidence: 0.940. Support: 242.` |
| 113 | `  -1.diff_col ≤ 2<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, const, {}<br>	∧ -2.diff_offset ≥ 5<br>	∧ -3.reserved not in {,}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles in {TYPE} and not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.950. Support: 191.` |
| 114 | `  -1.diff_col ≤ 2<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, const, {}<br>	∧ -3.reserved not in {,}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.937. Support: 5765.` |
| 115 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, const, {}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ -3.reserved not in {,}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.993. Support: 11317.` |
| 116 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, const, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -3.reserved not in {,}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.963. Support: 5924.` |
| 117 | `  •••start_col ≥ 8<br>	∧ -1.reserved = ,<br>	∧ +1.roles in {EXPRESSION}<br>	∧ +4.reserved = ,<br>	∧ ^1.roles in {MAP}<br>⇒ y = ⏎<br>Confidence: 0.997. Support: 32666.` |
| 118 | `  •••start_col ≥ 8<br>	∧ -1.reserved = ,<br>	∧ -3.length ≥ 7<br>	∧ +1.roles in {EXPRESSION}<br>	∧ +4.reserved not in {,}<br>	∧ ^1.roles in {MAP}<br>⇒ y = ⏎<br>Confidence: 0.977. Support: 451.` |
| 119 | `  -1.reserved not in {,}<br>	∧ -1.roles in {STRING}<br>	∧ -3.reserved = ,<br>	∧ +4.roles in {KEY}<br>⇒ y = "<br>Confidence: 0.988. Support: 538.` |
| 120 | `  •••start_col ≤ 20<br>	∧ •••start_line ≤ 30<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ;<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>⇒ y = ⏎⏎<br>Confidence: 0.944. Support: 278.` |
| 121 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ -2.label in {<space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +2.reserved not in {}}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.938. Support: 1902.` |
| 122 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, ;, {}<br>	∧ -2.label not in {<-space>}<br>	∧ -3.reserved not in {,}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = =<br>	∧ ^1.roles not in {INCOMPLETE}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 306.` |
| 123 | `  •••start_line ≤ 12<br>	∧ -1.internal_type not in {CommentLine}<br>	∧ -1.reserved not in {,, :, ;, {}<br>	∧ -2.label not in {<-space>}<br>	∧ -3.reserved not in {,}<br>	∧ -4.diff_offset ≥ 244<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {{}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = '<br>Confidence: 0.991. Support: 158.` |
| 124 | `  -1.diff_col ≤ 2<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, const, {}<br>	∧ -2.label not in {<space>}<br>	∧ -3.reserved not in {,}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.940. Support: 5687.` |
| 125 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, const, {}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ -3.reserved not in {,}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.996. Support: 11456.` |
| 126 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, const, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -3.reserved not in {,}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.954. Support: 6090.` |
| 127 | `  -1.reserved not in {,}<br>	∧ -1.roles in {STRING}<br>	∧ -3.reserved = ,<br>	∧ +4.roles in {MAP}<br>⇒ y = "<br>Confidence: 0.982. Support: 568.` |
| 128 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, {}<br>	∧ -3.reserved not in {,}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = {<br>	∧ +2.roles not in {KEY}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.957. Support: 1352.` |
| 129 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, ;, {}<br>	∧ -2.label not in {<-space>}<br>	∧ -3.reserved not in {,}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = from<br>⇒ y = ␣<br>Confidence: 0.975. Support: 139.` |
| 130 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, import}<br>	∧ -2.label not in {<-space>}<br>	∧ -3.reserved not in {,}<br>	∧ -5.roles in {MAP}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {{}<br>	∧ +2.internal_type = TemplateElement<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.977. Support: 152.` |
| 131 | `  -1.diff_col ≥ 3<br>	∧ -1.diff_offset ≥ 12<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, {}<br>	∧ -3.reserved not in {,}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.978. Support: 383.` |
| 132 | `  -1.diff_col ≤ 2<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved = (<br>	∧ -2.label not in {<space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=}<br>	∧ +1.length ≥ 2<br>	∧ +2.reserved = )<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 429.` |
| 133 | `  -1.diff_col ≤ 2<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {(, ,, :, ;, {}<br>	∧ -2.label not in {<space>}<br>	∧ -3.reserved not in {,}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.959. Support: 5000.` |
| 134 | `  -1.reserved not in {,, :}<br>	∧ -2.label not in {<space>}<br>	∧ -3.reserved not in {,}<br>	∧ -3.roles not in {VALUE}<br>	∧ +1.internal_type = StringLiteral<br>	∧ +3.roles not in {STRING}<br>⇒ y = '<br>Confidence: 0.928. Support: 2761.` |
| 135 | `  -1.diff_offset ≥ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, {}<br>	∧ -3.reserved not in {,}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = {<br>	∧ +2.roles in {MAP}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 154.` |
| 136 | `  •••start_line ≤ 10<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, :, if, {}<br>	∧ -2.label not in {<-space>}<br>	∧ -2.roles in {COMMENT}<br>	∧ -3.reserved not in {,}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {{}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ⏎⏎<br>Confidence: 0.943. Support: 168.` |
| 137 | `  •••start_col ≥ 5<br>	∧ -1.diff_col ≥ 3<br>	∧ -1.diff_offset ≤ 8<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, ;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.label not in {<-space>}<br>	∧ -3.reserved not in {,}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>⇒ y = ␣<br>Confidence: 0.943. Support: 799.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 9.664233576642335, "max_conf": 0.9993215799331665, "max_support": 36780, "min_conf": 0.9212328791618347, "min_support": 135, "num_rules": 137}}
```
</details>
