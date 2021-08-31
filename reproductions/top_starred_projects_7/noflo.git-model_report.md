# Model report for file:///tmp/top-repos-quality-repos-ogwqqd8r/noflo.git HEAD 6187566f761a463af95dd186de55dc488e2f03b8

### Dump

```json
{'created_at': '2021-08-30 17:34:52',
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
 'uuid': '55f3df18-a187-42f4-8b43-f289213f17ec',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-ogwqqd8r/noflo.git 6187566f761a463af95dd186de55dc488e2f03b8

# javascript
177 rules, avg.len. 8.9
## train
PPCR: 0.983396
### report
macro
{'f1-score': 0.7619472783882344,
 'precision': 0.7992698386372097,
 'recall': 0.749416882833921,
 'support': 91917}
micro
{'f1-score': 0.9661651272343528,
 'precision': 0.9661651272343528,
 'recall': 0.9661651272343528,
 'support': 91917}
weighted
{'f1-score': 0.9645184952133208,
 'precision': 0.9642603693672954,
 'recall': 0.9661651272343528,
 'support': 91917}
### report_full
macro
{'f1-score': 0.7459827420514105,
 'precision': 0.7992698386372097,
 'recall': 0.7243790164009429,
 'support': 93469}
micro
{'f1-score': 0.9580766616680871,
 'precision': 0.9661651272343528,
 'recall': 0.9501225005081899,
 'support': 93469}
weighted
{'f1-score': 0.9556621450533328,
 'precision': 0.9634679267673255,
 'recall': 0.9501225005081899,
 'support': 93469}
## test
PPCR: 0.991876
### report
macro
{'f1-score': 0.7242702583356793,
 'precision': 0.7956780119557201,
 'recall': 0.694596758868939,
 'support': 24663}
micro
{'f1-score': 0.9517495843976808,
 'precision': 0.9517495843976808,
 'recall': 0.9517495843976808,
 'support': 24663}
weighted
{'f1-score': 0.948762308734578,
 'precision': 0.948082176402116,
 'recall': 0.9517495843976808,
 'support': 24663}
### report_full
macro
{'f1-score': 0.7137696398438854,
 'precision': 0.7956780119557201,
 'recall': 0.6813658191630758,
 'support': 24865}
micro
{'f1-score': 0.9478678727184624,
 'precision': 0.9517495843976808,
 'recall': 0.9440176955560025,
 'support': 24865}
weighted
{'f1-score': 0.9444505453241478,
 'precision': 0.9478846849702297,
 'recall': 0.9440176955560025,
 'support': 24865}
```

## javascript
### Summary
111 rules, avg.len. 9.2

| | |
|-|-|
|Min support|188|
|Max support|21000|
|Min confidence|0.9200235605239868|
|Max confidence|0.9998428225517273|

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
                     'min_samples_leaf': 120,
                     'min_samples_split': 237,
                     'n_estimators': 10,
                     'prune_attributes': True,
                     'prune_branches_algorithms': ['reduced-error'],
                     'prune_dataset_ratio': 0.2,
                     'top_down_greedy_budget': [False, 0.5]}}
```

### Rules

| rule number | description |
|----:|:-----|
| 1 | `  ^1.roles in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.990. Support: 16903.` |
| 2 | `  -1.internal_type = StringLiteral<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = '<br>Confidence: 1.000. Support: 3181.` |
| 3 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ +1.internal_type = StringLiteral<br>	∧ ^1.internal_type not in {ArrayExpression}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.988. Support: 449.` |
| 4 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = :<br>	∧ +1.internal_type = StringLiteral<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 350.` |
| 5 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,}<br>	∧ -2.label not in {<space>}<br>	∧ +1.internal_type = StringLiteral<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = '<br>Confidence: 0.966. Support: 3321.` |
| 6 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;, {}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = =<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 1.000. Support: 2545.` |
| 7 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 3468.` |
| 8 | `  -1.internal_type = CommentLine<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ +1.reserved not in {=, {}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type = File<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ⏎<br>Confidence: 0.972. Support: 633.` |
| 9 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {}<br>	∧ +1.roles in {COMMENT}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type = File<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 809.` |
| 10 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles in {COMMENT}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type = File<br>⇒ y = ∅<br>Confidence: 0.998. Support: 282.` |
| 11 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles in {LITERAL} and not in {IDENTIFIER}<br>⇒ y = ⏎<br>Confidence: 0.940. Support: 405.` |
| 12 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ,, ;, {}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles in {LITERAL} and not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 413.` |
| 13 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -1.length ≥ 2<br>	∧ -2.reserved = (<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {{}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles not in {IDENTIFIER, LITERAL}<br>⇒ y = ␣<br>Confidence: 0.974. Support: 331.` |
| 14 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -1.length ≤ 1<br>	∧ -2.reserved = (<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles not in {IDENTIFIER, LITERAL}<br>⇒ y = ∅<br>Confidence: 0.971. Support: 296.` |
| 15 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -2.reserved not in {(}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {{}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles not in {IDENTIFIER, LITERAL}<br>⇒ y = ␣<br>Confidence: 0.920. Support: 4245.` |
| 16 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = (<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.948. Support: 700.` |
| 17 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = if<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 712.` |
| 18 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;, if}<br>	∧ -3.reserved = ;<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = }<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.959. Support: 327.` |
| 19 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, }}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 275.` |
| 20 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;, if, {}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {CONDITION, IDENTIFIER}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.985. Support: 20865.` |
| 21 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ +1.internal_type = StringLiteral<br>	∧ ^1.roles not in {IDENTIFIER, LIST}<br>⇒ y = ␣<br>Confidence: 0.988. Support: 383.` |
| 22 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;}<br>	∧ -2.label not in {<space>}<br>	∧ +1.internal_type = StringLiteral<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = '<br>Confidence: 0.971. Support: 3219.` |
| 23 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +4.reserved = }<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.940. Support: 193.` |
| 24 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = {<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.961. Support: 191.` |
| 25 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = {<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.953. Support: 2578.` |
| 26 | `  -1.internal_type = CommentLine<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ +1.reserved not in {=, {}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles in {FILE} and not in {IDENTIFIER}<br>⇒ y = ⏎<br>Confidence: 0.979. Support: 599.` |
| 27 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -2.internal_type not in {CommentBlock}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {}<br>	∧ +1.roles in {COMMENT}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles in {FILE}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 791.` |
| 28 | `  -1.diff_col ≥ 10<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -2.internal_type not in {CommentBlock}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles in {FILE}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 305.` |
| 29 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ,, ;, {}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles in {LITERAL} and not in {FILE, IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 408.` |
| 30 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -1.length ≥ 2<br>	∧ -2.reserved = (<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {{}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {FILE, IDENTIFIER, LITERAL}<br>⇒ y = ␣<br>Confidence: 0.983. Support: 325.` |
| 31 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -1.length ≤ 1<br>	∧ -2.reserved = (<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {FILE, IDENTIFIER, LITERAL}<br>⇒ y = ∅<br>Confidence: 0.975. Support: 302.` |
| 32 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;, if, {}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.988. Support: 21000.` |
| 33 | `  ^1.roles in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.987. Support: 16566.` |
| 34 | `  -1.internal_type = StringLiteral<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = '<br>Confidence: 1.000. Support: 2981.` |
| 35 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ +1.internal_type = StringLiteral<br>	∧ ^1.internal_type not in {ArrayExpression}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.992. Support: 416.` |
| 36 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = :<br>	∧ +1.internal_type = StringLiteral<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 336.` |
| 37 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;}<br>	∧ -2.label not in {<space>}<br>	∧ +1.internal_type = StringLiteral<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = '<br>Confidence: 0.972. Support: 3312.` |
| 38 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +4.reserved = }<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.945. Support: 190.` |
| 39 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;, {}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = =<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 1.000. Support: 2697.` |
| 40 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 1.000. Support: 3448.` |
| 41 | `  -1.internal_type = CommentLine<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ +1.reserved not in {=, {}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles in {FILE} and not in {QUALIFIED}<br>⇒ y = ⏎<br>Confidence: 0.986. Support: 603.` |
| 42 | `  -1.diff_offset ≥ 10<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -2.internal_type not in {CommentBlock}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles in {FILE}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 287.` |
| 43 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles in {LITERAL} and not in {FILE, QUALIFIED}<br>⇒ y = ⏎<br>Confidence: 0.943. Support: 398.` |
| 44 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ,, ;, {}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles in {LITERAL} and not in {FILE, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 425.` |
| 45 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -1.length ≥ 2<br>	∧ -2.reserved = (<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {{}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {FILE, LITERAL, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.979. Support: 316.` |
| 46 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -1.length ≤ 2<br>	∧ -2.reserved = (<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {FILE, LITERAL, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.970. Support: 313.` |
| 47 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -2.reserved not in {(}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {{}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {FILE, LITERAL, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.921. Support: 4404.` |
| 48 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = }<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.994. Support: 432.` |
| 49 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = if<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 647.` |
| 50 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;, if}<br>	∧ -3.reserved = ;<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = }<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.981. Support: 334.` |
| 51 | `  •••start_col ≥ 18<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;, if, {}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED, VARIABLE}<br>⇒ y = ∅<br>Confidence: 0.921. Support: 350.` |
| 52 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;, if, {}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED, VARIABLE}<br>⇒ y = ∅<br>Confidence: 0.985. Support: 20793.` |
| 53 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ +1.internal_type = StringLiteral<br>	∧ ^1.roles not in {LITERAL, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.989. Support: 426.` |
| 54 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = {<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.923. Support: 188.` |
| 55 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = {<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.961. Support: 2620.` |
| 56 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -2.roles not in {COMMENT}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {}<br>	∧ +1.roles in {COMMENT}<br>	∧ +1.length ≥ 2<br>	∧ +2.length ≤ 18<br>	∧ ^1.roles in {FILE}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 788.` |
| 57 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.length ≥ 10<br>	∧ -2.roles not in {COMMENT}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {}<br>	∧ +1.length ≥ 2<br>	∧ +2.length ≤ 18<br>	∧ ^1.roles in {FILE}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 261.` |
| 58 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -4.diff_offset ≤ 5<br>	∧ -5.label in {<newline>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {FILE, LITERAL, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.928. Support: 201.` |
| 59 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = (<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.965. Support: 747.` |
| 60 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, }}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 251.` |
| 61 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;, if, {}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.985. Support: 20719.` |
| 62 | `  ^1.internal_type = MemberExpression<br>⇒ y = ∅<br>Confidence: 0.990. Support: 16862.` |
| 63 | `  -1.internal_type = StringLiteral<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = '<br>Confidence: 1.000. Support: 3059.` |
| 64 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ +1.internal_type = StringLiteral<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {LITERAL}<br>⇒ y = ␣<br>Confidence: 0.992. Support: 416.` |
| 65 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = :<br>	∧ +1.internal_type = StringLiteral<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 354.` |
| 66 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;}<br>	∧ -2.label not in {<space>}<br>	∧ +1.internal_type = StringLiteral<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = '<br>Confidence: 0.975. Support: 3216.` |
| 67 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;, {}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = =<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ␣<br>Confidence: 1.000. Support: 2592.` |
| 68 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 3342.` |
| 69 | `  -1.internal_type = CommentLine<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ +1.reserved not in {=, {}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type = File<br>⇒ y = ⏎<br>Confidence: 0.968. Support: 668.` |
| 70 | `  -1.diff_offset ≥ 2<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -2.roles not in {BLOCK}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {}<br>	∧ +1.roles in {COMMENT}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type = File<br>⇒ y = ∅<br>Confidence: 0.999. Support: 810.` |
| 71 | `  -1.diff_offset ≥ 2<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.length ≥ 30<br>	∧ -2.roles not in {BLOCK}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type = File<br>⇒ y = ∅<br>Confidence: 0.998. Support: 247.` |
| 72 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {File, MemberExpression}<br>	∧ ^1.roles in {LITERAL}<br>⇒ y = ⏎<br>Confidence: 0.927. Support: 389.` |
| 73 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ,, ;, {}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {File, MemberExpression}<br>	∧ ^1.roles in {LITERAL}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 448.` |
| 74 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = (<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ␣<br>Confidence: 0.955. Support: 797.` |
| 75 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = if<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 608.` |
| 76 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;, if}<br>	∧ -3.reserved = ;<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = }<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.947. Support: 309.` |
| 77 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, }}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 257.` |
| 78 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;, if, {}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR, VARIABLE}<br>⇒ y = ∅<br>Confidence: 0.987. Support: 20752.` |
| 79 | `  -1.internal_type = CommentLine<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ +1.reserved not in {=, {}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type = File<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ⏎<br>Confidence: 0.977. Support: 579.` |
| 80 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -2.roles not in {COMMENT}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {}<br>	∧ +1.roles in {COMMENT}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type = File<br>⇒ y = ∅<br>Confidence: 0.999. Support: 801.` |
| 81 | `  -1.diff_col ≥ 10<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -2.roles not in {COMMENT}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type = File<br>⇒ y = ∅<br>Confidence: 0.998. Support: 279.` |
| 82 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles in {LITERAL} and not in {QUALIFIED}<br>⇒ y = ⏎<br>Confidence: 0.929. Support: 390.` |
| 83 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ,, ;, {}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles in {LITERAL} and not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 428.` |
| 84 | `  -1.diff_offset ≥ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -2.reserved = (<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {{}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles not in {LITERAL, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.977. Support: 366.` |
| 85 | `  -1.diff_offset ≤ 1<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -2.reserved = (<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles not in {LITERAL, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.951. Support: 340.` |
| 86 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;, if}<br>	∧ -4.diff_line ≥ 1<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = }<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.932. Support: 359.` |
| 87 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, }}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 236.` |
| 88 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;, if, {}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.988. Support: 20905.` |
| 89 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -2.roles not in {BLOCK}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {}<br>	∧ +1.roles in {COMMENT}<br>	∧ +1.length ≥ 2<br>	∧ +2.length ≤ 18<br>	∧ ^1.internal_type = File<br>⇒ y = ∅<br>Confidence: 0.999. Support: 778.` |
| 90 | `  •••start_col ≥ 30<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -2.roles not in {BLOCK}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {}<br>	∧ +1.length ≥ 2<br>	∧ +2.length ≤ 18<br>	∧ ^1.internal_type = File<br>⇒ y = ∅<br>Confidence: 0.998. Support: 261.` |
| 91 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -1.length ≥ 2<br>	∧ -2.reserved = (<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {{}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {File, MemberExpression}<br>	∧ ^1.roles not in {LITERAL}<br>⇒ y = ␣<br>Confidence: 0.976. Support: 357.` |
| 92 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -1.length ≤ 2<br>	∧ -2.reserved = (<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {File, MemberExpression}<br>	∧ ^1.roles not in {LITERAL}<br>⇒ y = ∅<br>Confidence: 0.970. Support: 314.` |
| 93 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {CallExpression, MemberExpression}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.924. Support: 535.` |
| 94 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;, if}<br>	∧ -4.diff_line ≥ 1<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = }<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.946. Support: 322.` |
| 95 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;, if, {}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.985. Support: 20644.` |
| 96 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ +1.internal_type = StringLiteral<br>	∧ ^1.roles not in {LIST, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.988. Support: 446.` |
| 97 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {}<br>	∧ +1.roles in {COMMENT}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type = File<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 837.` |
| 98 | `  -1.diff_offset ≥ 10<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type = File<br>⇒ y = ∅<br>Confidence: 0.998. Support: 262.` |
| 99 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -2.diff_col ≥ 3<br>	∧ -2.reserved = (<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {{}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles not in {LITERAL, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.980. Support: 326.` |
| 100 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -2.diff_col ≤ 3<br>	∧ -2.reserved = (<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles not in {LITERAL, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.978. Support: 292.` |
| 101 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {CallExpression}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.931. Support: 486.` |
| 102 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ +1.internal_type = StringLiteral<br>	∧ ^1.roles not in {IDENTIFIER, LITERAL}<br>⇒ y = ␣<br>Confidence: 0.996. Support: 413.` |
| 103 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -1.length ≥ 2<br>	∧ -2.reserved = (<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {{}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles not in {IDENTIFIER, LITERAL}<br>⇒ y = ␣<br>Confidence: 0.970. Support: 321.` |
| 104 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -1.length ≤ 2<br>	∧ -2.reserved = (<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles not in {IDENTIFIER, LITERAL}<br>⇒ y = ∅<br>Confidence: 0.965. Support: 329.` |
| 105 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = {<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ∅<br>Confidence: 0.949. Support: 207.` |
| 106 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = {<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ␣<br>Confidence: 0.956. Support: 2606.` |
| 107 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {}<br>	∧ +1.roles in {COMMENT}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type = File<br>⇒ y = ∅<br>Confidence: 0.999. Support: 725.` |
| 108 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.length ≥ 10<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type = File<br>⇒ y = ∅<br>Confidence: 0.998. Support: 268.` |
| 109 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -2.diff_offset ≥ 3<br>	∧ -2.reserved = (<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {{}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {File, MemberExpression}<br>	∧ ^1.roles not in {LITERAL}<br>⇒ y = ␣<br>Confidence: 0.954. Support: 316.` |
| 110 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -2.diff_offset ≤ 2<br>	∧ -2.reserved = (<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {File, MemberExpression}<br>	∧ ^1.roles not in {LITERAL}<br>⇒ y = ∅<br>Confidence: 0.970. Support: 279.` |
| 111 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, }}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR, VARIABLE}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 248.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 9.162162162162161, "max_conf": 0.9998428225517273, "max_support": 21000, "min_conf": 0.9200235605239868, "min_support": 188, "num_rules": 111}}
```
</details>
