# Model report for file:///tmp/top-repos-quality-repos-gu2dgx8k/carbon.git HEAD 02ddcbdcb34e01efdf500cc4b55fc0d5dc6feefc

### Dump

```json
{'created_at': '2021-08-29 21:57:15',
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
 'size': '17.9 kB',
 'tags': [],
 'uuid': '17d53269-3bc0-454f-8f29-8c797ce5382d',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-gu2dgx8k/carbon.git 02ddcbdcb34e01efdf500cc4b55fc0d5dc6feefc

# javascript
41 rules, avg.len. 8.2
## train
PPCR: 0.853328
### report
macro
{'f1-score': 0.8703724583159836,
 'precision': 0.9264594862583446,
 'recall': 0.8290519191955881,
 'support': 33808}
micro
{'f1-score': 0.9435636535731188,
 'precision': 0.9435636535731188,
 'recall': 0.9435636535731188,
 'support': 33808}
weighted
{'f1-score': 0.9420276319491714,
 'precision': 0.9427460147457223,
 'recall': 0.9435636535731188,
 'support': 33808}
### report_full
macro
{'f1-score': 0.7155797766491541,
 'precision': 0.9264594862583446,
 'recall': 0.6328464876540284,
 'support': 39619}
micro
{'f1-score': 0.86889019025699,
 'precision': 0.9435636535731188,
 'recall': 0.805169236982256,
 'support': 39619}
weighted
{'f1-score': 0.8549267917143193,
 'precision': 0.9384811887454709,
 'recall': 0.805169236982256,
 'support': 39619}
## test
PPCR: 0.808311
### report
macro
{'f1-score': 0.8888274059324626,
 'precision': 0.9612236629106081,
 'recall': 0.8353950358496809,
 'support': 9454}
micro
{'f1-score': 0.9611804527184261,
 'precision': 0.9611804527184261,
 'recall': 0.9611804527184261,
 'support': 9454}
weighted
{'f1-score': 0.9602624075709862,
 'precision': 0.9611167063317911,
 'recall': 0.9611804527184261,
 'support': 9454}
### report_full
macro
{'f1-score': 0.6639988094752463,
 'precision': 0.9612236629106081,
 'recall': 0.5718132722322531,
 'support': 11696}
micro
{'f1-score': 0.8592907801418439,
 'precision': 0.9611804527184261,
 'recall': 0.7769322845417237,
 'support': 11696}
weighted
{'f1-score': 0.8286675750247564,
 'precision': 0.9601060141116565,
 'recall': 0.7769322845417237,
 'support': 11696}
```

## javascript
### Summary
26 rules, avg.len. 7.4

| | |
|-|-|
|Min support|91|
|Max support|12878|
|Min confidence|0.9203628897666931|
|Max confidence|0.9989224076271057|

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
| 1 | `  -1.internal_type = StringLiteral<br>	∧ -3.internal_type = JSXIdentifier<br>⇒ y = "<br>Confidence: 0.999. Support: 464.` |
| 2 | `  -1.internal_type = StringLiteral<br>	∧ -3.internal_type not in {JSXIdentifier}<br>⇒ y = '<br>Confidence: 0.995. Support: 1274.` |
| 3 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.internal_type = StringLiteral<br>	∧ ^2.internal_type = JSXOpeningElement<br>⇒ y = "<br>Confidence: 0.999. Support: 464.` |
| 4 | `  -1.internal_type not in {StringLiteral}<br>	∧ -2.label in {<space>}<br>	∧ +1.internal_type = StringLiteral<br>	∧ ^2.internal_type not in {JSXOpeningElement}<br>⇒ y = ␣<br>Confidence: 0.968. Support: 423.` |
| 5 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ -2.label not in {<space>}<br>	∧ +1.internal_type = StringLiteral<br>	∧ +4.reserved = ,<br>	∧ ^2.internal_type not in {JSXOpeningElement}<br>⇒ y = ⏎<br>Confidence: 0.989. Support: 238.` |
| 6 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = :<br>	∧ -2.label not in {<space>}<br>	∧ +1.internal_type = StringLiteral<br>	∧ ^2.internal_type not in {JSXOpeningElement}<br>⇒ y = ␣<br>Confidence: 0.985. Support: 166.` |
| 7 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :}<br>	∧ -2.diff_col ≤ 15<br>	∧ -2.label not in {<space>}<br>	∧ +1.internal_type = StringLiteral<br>	∧ ^2.internal_type not in {JSXOpeningElement}<br>⇒ y = '<br>Confidence: 0.980. Support: 1272.` |
| 8 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {{}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = )<br>	∧ ^1.roles in {DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.994. Support: 258.` |
| 9 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {{}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = ><br>	∧ ^1.roles in {DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 209.` |
| 10 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = ,<br>	∧ ^1.roles in {DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.996. Support: 125.` |
| 11 | `  -1.internal_type = Identifier<br>	∧ -1.reserved not in {(, {}<br>	∧ +1.reserved = (<br>	∧ ^1.roles in {DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 165.` |
| 12 | `  -1.internal_type not in {Identifier, StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = (<br>	∧ ^1.roles in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.969. Support: 145.` |
| 13 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +2.roles in {EXPRESSION}<br>	∧ ^1.roles in {OPERATOR} and not in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.981. Support: 437.` |
| 14 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = )<br>	∧ +1.roles not in {IDENTIFIER}<br>	∧ ^1.roles in {BLOCK} and not in {DECLARATION, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.966. Support: 394.` |
| 15 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = }<br>	∧ +1.roles not in {IDENTIFIER}<br>	∧ ^1.internal_type not in {JSXOpeningElement}<br>	∧ ^1.roles in {BLOCK} and not in {DECLARATION, OPERATOR}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.930. Support: 235.` |
| 16 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), }}<br>	∧ +1.roles not in {IDENTIFIER}<br>	∧ +2.reserved = )<br>	∧ ^1.internal_type not in {JSXOpeningElement}<br>	∧ ^1.roles in {BLOCK} and not in {DECLARATION, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.969. Support: 113.` |
| 17 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = (<br>	∧ +1.roles not in {IDENTIFIER}<br>	∧ ^1.internal_type not in {JSXOpeningElement}<br>	∧ ^1.roles in {BLOCK} and not in {DECLARATION, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.973. Support: 91.` |
| 18 | `  -1.diff_col ≥ 5<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.roles in {MODULE} and not in {BLOCK, DECLARATION, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.920. Support: 496.` |
| 19 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = :<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.roles not in {BLOCK, DECLARATION, MODULE, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.985. Support: 361.` |
| 20 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.roles in {IF} and not in {BLOCK, DECLARATION, MODULE, OPERATOR}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.930. Support: 151.` |
| 21 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, {}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = )<br>	∧ ^1.roles in {IF} and not in {BLOCK, DECLARATION, MODULE, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.955. Support: 168.` |
| 22 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, {}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {)}<br>	∧ ^1.roles in {IDENTIFIER, IF} and not in {BLOCK, DECLARATION, MODULE, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.996. Support: 140.` |
| 23 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, {}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = {<br>	∧ ^1.roles in {IF} and not in {BLOCK, BODY, DECLARATION, IDENTIFIER, MODULE}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 144.` |
| 24 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, {}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), {}<br>	∧ +2.roles in {IF}<br>	∧ ^1.roles in {IF} and not in {BLOCK, BODY, DECLARATION, IDENTIFIER, MODULE}<br>⇒ y = ␣<br>Confidence: 0.978. Support: 114.` |
| 25 | `  -1.internal_type = CommentLine<br>	∧ -1.reserved not in {,, :}<br>	∧ +1.roles not in {MAP}<br>	∧ ^1.roles not in {BLOCK, DECLARATION, IF, MODULE, OPERATOR}<br>	∧ ^2.roles not in {TYPE}<br>⇒ y = ⏎<br>Confidence: 0.927. Support: 131.` |
| 26 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {), ,, :, }}<br>	∧ -2.internal_type not in {StringLiteral}<br>	∧ -5.diff_offset ≥ 6<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {MAP}<br>	∧ ^1.roles not in {BLOCK, DECLARATION, IF, MODULE, OPERATOR}<br>	∧ ^2.roles not in {TYPE}<br>⇒ y = ∅<br>Confidence: 0.963. Support: 12878.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 7.423076923076923, "max_conf": 0.9989224076271057, "max_support": 12878, "min_conf": 0.9203628897666931, "min_support": 91, "num_rules": 26}}
```
</details>
